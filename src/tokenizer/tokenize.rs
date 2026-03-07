//! Tokenizer with a function to intern strings

use crate::compiler::span::Span;
use crate::compiler::symbol::SymbolFactory;
use crate::diagnostic::stream::DiagnosticStream;
use crate::tokenizer::errors::{TokenizeErr, TokenizeErrKind};
use crate::tokenizer::generated_tokenmap::{
    scan_operator_or_delimiter, scan_short_keywords, LONG_KEYWORDS_MAP,
};
use crate::tokenizer::tokens::{Comment, Literal, Token};

type WithSpanVec<T> = Vec<(T, Span)>;

pub struct Tokenizer<'src, 'ctx> {
    pub(super) current_pos: usize,
    line_starts: Vec<u32>,
    input: &'src [u8],
    symbol_factory: &'ctx mut SymbolFactory<'src>,
}

impl<'src, 'ctx> Tokenizer<'src, 'ctx> {
    pub fn new(
        input: &'src str,
        symbol_factory: &'ctx mut SymbolFactory<'src>,
    ) -> Tokenizer<'src, 'ctx> {
        Self {
            current_pos: 0,
            input: input.as_bytes(),
            symbol_factory,
            line_starts: Vec::new(),
        }
    }

    pub fn tokenize(
        mut self,
        mut diagnostic_stream: &mut impl DiagnosticStream,
    ) -> (WithSpanVec<Token>, Vec<u32>) {
        let mut tokens = Vec::with_capacity(self.input.len() / 4); // expect we need length/4 vector
        let mut error_count = 0;

        while let Some(b) = self.peek() {
            let begin = self.now_pos();

            let next = match b {
                b' ' | b'\t' | b'\r' => {
                    self.advance(); // TODO: make it first by simd
                    continue;
                }
                b'\n' => {
                    self.advance();
                    self.line_starts.push(self.current_pos as u32);
                    continue;
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.read_identifier_or_keyword(),
                b'0'..=b'9' => self.read_number_literal(),
                b'"' => self.read_string_literal(),
                b'\'' => self.read_char_literal(),
                b'/' if self.peek_at(1) == Some(b'/') => self.read_line_comment(),
                b'/' if self.peek_at(1) == Some(b'*') => self.read_block_comment(),
                _ => {
                    let token = self.read_operator_or_delimiter();
                    if token.is_err() {
                        self.advance_utf8_char();
                    }
                    token
                }
            };

            match next {
                Ok(token) => tokens.push((token, Span::new(begin, self.now_pos()))),
                Err(err) => {
                    if error_count < 100 {
                        diagnostic_stream
                            .pour(TokenizeErr::new(err, Span::new(begin, self.now_pos())));
                        error_count += 1;
                    }
                    tokens.push((Token::Invalid, Span::new(begin, self.now_pos())));
                }
            }
        }

        tokens.push((Token::EndOfFile, Span::new(self.now_pos(), self.now_pos())));

        (tokens, self.line_starts)
    }

    // --- low level helpers ---

    #[inline(always)]
    pub(super) fn peek(&self) -> Option<u8> {
        self.input.get(self.current_pos).copied()
    }

    #[inline(always)]
    fn peek_at(&self, offset: usize) -> Option<u8> {
        self.input.get(self.current_pos + offset).copied()
    }

    #[inline(always)]
    pub(super) fn advance(&mut self) {
        self.current_pos += 1;
    }

    #[inline(always)]
    fn advance_n(&mut self, n: usize) {
        self.current_pos += n;
    }

    fn now_pos(&self) -> usize {
        self.current_pos
    }

    #[inline]
    fn consume_str(&mut self, target: &[u8]) -> bool {
        let len = target.len();
        if self.current_pos + len <= self.input.len()
            && &self.input[self.current_pos..self.current_pos + len] == target
        {
            self.current_pos += len;
            true
        } else {
            false
        }
    }

    // --- logic to read each tokens ---

    fn read_identifier_or_keyword(&mut self) -> Result<Token, TokenizeErrKind> {
        let begin = self.current_pos;
        let start = self.now_pos();
        while let Some(b) = self.peek() {
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.advance();
            } else {
                break;
            }
        }
        let slice = &self.input[begin..self.current_pos];

        if slice.len() < 5 {
            match scan_short_keywords(slice) {
                Token::Invalid => {
                    let symbol = self.symbol_factory.from_range(start, self.now_pos());
                    Ok(Token::Identifier(symbol))
                }
                token => Ok(token),
            }
        } else {
            match LONG_KEYWORDS_MAP.get(slice) {
                Some(&keyword) => Ok(keyword),
                None => {
                    let symbol = self.symbol_factory.from_range(start, self.now_pos());
                    Ok(Token::Identifier(symbol))
                }
            }
        }
    }

    fn read_number_literal(&mut self) -> Result<Token, TokenizeErrKind> {
        let start = self.current_pos;
        let mut is_float = false;

        if self.consume_str(b"0x") {
            // prefix for Hexadecimal
            while let Some(b) = self.peek() {
                if b.is_ascii_hexdigit() {
                    self.advance();
                } else {
                    break;
                }
            }
        } else if self.consume_str(b"0b") {
            // prefix for binary notation
            while let Some(b) = self.peek() {
                if b == b'0' || b == b'1' {
                    self.advance();
                } else {
                    break;
                }
            }
        } else {
            while let Some(b) = self.peek() {
                if b.is_ascii_digit() {
                    self.advance();
                } else if b == b'.' {
                    if let Some(next) = self.peek_at(1) {
                        if next == b'.' {
                            break;
                        } // for ".." operator
                    }
                    is_float = true;
                    self.advance();
                } else if b == b'e' || b == b'E' {
                    is_float = true;
                    self.advance();
                    if matches!(self.peek(), Some(b'+' | b'-')) {
                        self.advance();
                    }
                } else {
                    break;
                }
            }
        }

        let slice = unsafe { std::str::from_utf8_unchecked(&self.input[start..self.current_pos]) };

        // TODO: add logic to parse f64 and i64
        if is_float {
            slice
                .parse::<f32>()
                .map(|v| Token::Literal(Literal::FloatLiteral(v.into())))
                .map_err(|_| TokenizeErrKind::InvalidFloatLiteral)
        } else {
            slice
                .parse::<i32>()
                .map(|v| Token::Literal(Literal::IntegerLiteral(v)))
                .map_err(|_| TokenizeErrKind::InvalidIntegerLiteral)
        }
    }

    fn read_string_literal(&mut self) -> Result<Token, TokenizeErrKind> {
        self.advance(); // skip opening "
        let begin = self.current_pos;
        let start = self.now_pos();
        while let Some(b) = self.peek() {
            match b {
                b'"' => {
                    let span = Span::new(start, self.now_pos());
                    self.advance(); // skip closing "
                    return Ok(Token::Literal(Literal::StringLiteral(span)));
                }
                b'\\' => {
                    self.advance(); // \
                    self.advance_utf8_char();
                }
                _ => {
                    self.advance();
                }
            }
        }
        Err(TokenizeErrKind::StringLiteralNotClosed)
    }

    fn read_char_literal(&mut self) -> Result<Token, TokenizeErrKind> {
        self.advance(); // '
        let start = self.current_pos;
        let c = match self.peek() {
            Some(b'\\') => {
                self.advance();
                let esc = self.peek().ok_or(TokenizeErrKind::CharLiteralNotClosed)?;
                self.advance();
                match esc {
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'\\' => '\\',
                    b'\'' => '\'',
                    _ => return Err(TokenizeErrKind::InvalidCharLiteral),
                }
            }
            Some(_) => {
                let remaining = &self.input[self.current_pos..];
                let s = std::str::from_utf8(remaining)
                    .map_err(|_| TokenizeErrKind::InvalidCharLiteral)?;
                let c = s
                    .chars()
                    .next()
                    .ok_or(TokenizeErrKind::CharLiteralNotClosed)?;
                self.advance_n(c.len_utf8());
                c
            }
            None => return Err(TokenizeErrKind::CharLiteralNotClosed),
        };

        if self.peek() == Some(b'\'') {
            self.advance();
            Ok(Token::Literal(Literal::CharLiteral(c)))
        } else {
            Err(TokenizeErrKind::CharLiteralNotClosed)
        }
    }

    fn read_line_comment(&mut self) -> Result<Token, TokenizeErrKind> {
        self.advance_n(2); // //
        let is_doc = self.peek() == Some(b'/');
        if is_doc {
            self.advance();
        }

        let start = self.now_pos();
        // TODO: make it firster by simd
        while let Some(b) = self.peek() {
            if b == b'\n' {
                break;
            }
            self.advance();
        }

        if is_doc {
            Ok(Token::Comment(Comment::DocComment(Span::new(
                start,
                self.now_pos(),
            ))))
        } else {
            Ok(Token::Comment(Comment::LineComment))
        }
    }

    fn read_block_comment(&mut self) -> Result<Token, TokenizeErrKind> {
        let start_err = self.current_pos;
        self.advance_n(2); // skip /*
        let mut depth = 1;
        while let Some(b) = self.peek() {
            if b == b'/' && self.peek_at(1) == Some(b'*') {
                depth += 1;
                self.advance_n(2);
            } else if b == b'*' && self.peek_at(1) == Some(b'/') {
                depth -= 1;
                self.advance_n(2);
                if depth == 0 {
                    return Ok(Token::Comment(Comment::BlockComment));
                }
            } else {
                self.advance();
            }
        }
        Err(TokenizeErrKind::BlockCommentNotClosed)
    }

    fn read_operator_or_delimiter(&mut self) -> Result<Token, TokenizeErrKind> {
        scan_operator_or_delimiter(self)
    }

    fn advance_utf8_char(&mut self) {
        if let Some(&b) = self.input.get(self.current_pos) {
            let len = if b < 0x80 {
                1
            } else if b < 0xE0 {
                2
            } else if b < 0xF0 {
                3
            } else {
                4
            };
            self.current_pos = std::cmp::min(self.current_pos + len, self.input.len());
        }
    }
}
