//! Tokenizer with a function to intern strings

use crate::compiler::span::Span;
use crate::compiler::symbol::SymbolFactory;
use crate::tokenizer::errors::TokenizeErr;
use crate::tokenizer::generated_tokenmap::{
    scan_operator_or_delimiter, scan_short_keywords, LONG_KEYWORDS_MAP,
};
use crate::tokenizer::tokens::{Comment, Literal, Token};

type WithSpanVec<T> = Vec<(T, Span)>;

pub struct Tokenizer<'src, 'ctx> {
    pub(super) current_pos: usize,
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
        }
    }

    pub fn tokenize(mut self) -> (WithSpanVec<Token>, WithSpanVec<TokenizeErr>) {
        let mut tokens = Vec::with_capacity(self.input.len() / 4); // expect we need length/4 vector
        let mut errors = Vec::new();

        while let Some(b) = self.peek() {
            let begin = self.current_pos;

            let next = match b {
                b' ' | b'\n' | b'\t' | b'\r' => {
                    self.advance(); // TODO: make it first by simd
                    continue;
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.read_identifier_or_keyword(),
                b'0'..=b'9' => self.read_number_literal(),
                b'"' => self.read_string_literal(),
                b'\'' => self.read_char_literal(),
                b'/' if self.peek_at(1) == Some(b'/') => self.read_line_comment(),
                b'/' if self.peek_at(2) == Some(b'*') => self.read_block_comment(),
                _ => self.read_operator_or_delimiter(),
            };

            match next {
                Ok(token) => tokens.push((token, Span::new(begin, self.current_pos))),
                Err(err) => {
                    errors.push((err, Span::new(begin, self.current_pos)));
                    tokens.push((Token::Invalid, Span::new(begin, self.current_pos)));
                }
            }
        }

        tokens.push((
            Token::EndOfFile,
            Span::new(self.current_pos, self.current_pos),
        ));

        (tokens, errors)
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

    fn read_identifier_or_keyword(&mut self) -> Result<Token, TokenizeErr> {
        let start = self.current_pos;
        while let Some(b) = self.peek() {
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.advance();
            } else {
                break;
            }
        }
        let slice = &self.input[start..self.current_pos];

        if slice.len() < 5 {
            match scan_short_keywords(slice) {
                Token::Invalid => {
                    let symbol = self.symbol_factory.from_range(start, self.current_pos);
                    Ok(Token::Identifier(symbol))
                }
                token => Ok(token),
            }
        } else {
            match LONG_KEYWORDS_MAP.get(slice) {
                Some(&keyword) => Ok(keyword),
                None => {
                    let symbol = self.symbol_factory.from_range(start, self.current_pos);
                    Ok(Token::Identifier(symbol))
                }
            }
        }
    }

    fn read_number_literal(&mut self) -> Result<Token, TokenizeErr> {
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
                .map_err(|_| TokenizeErr::InvalidFloatLiteral(start))
        } else {
            slice
                .parse::<i32>()
                .map(|v| Token::Literal(Literal::IntegerLiteral(v)))
                .map_err(|_| TokenizeErr::InvalidIntegerLiteral(start))
        }
    }

    fn read_string_literal(&mut self) -> Result<Token, TokenizeErr> {
        self.advance(); // skip opening "
        let start = self.current_pos;
        while let Some(b) = self.peek() {
            match b {
                b'"' => {
                    let span = Span::new(start, self.current_pos);
                    self.advance(); // skip closing "
                    return Ok(Token::Literal(Literal::StringLiteral(span)));
                }
                b'\\' => {
                    self.advance_n(2);
                }
                _ => {
                    self.advance();
                }
            }
        }
        Err(TokenizeErr::StringLiteralNotClosed(start))
    }

    fn read_char_literal(&mut self) -> Result<Token, TokenizeErr> {
        self.advance(); // '
        let start = self.current_pos;
        let c = match self.peek() {
            Some(b'\\') => {
                self.advance();
                let esc = self
                    .peek()
                    .ok_or(TokenizeErr::CharLiteralNotClosed(start))?;
                self.advance();
                match esc {
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'\\' => '\\',
                    b'\'' => '\'',
                    _ => return Err(TokenizeErr::InvalidCharLiteral(start)),
                }
            }
            Some(b) => {
                self.advance();
                b as char
            }
            None => return Err(TokenizeErr::CharLiteralNotClosed(start)),
        };

        if self.peek() == Some(b'\'') {
            self.advance();
            Ok(Token::Literal(Literal::CharLiteral(c)))
        } else {
            Err(TokenizeErr::CharLiteralNotClosed(start))
        }
    }

    fn read_line_comment(&mut self) -> Result<Token, TokenizeErr> {
        self.advance_n(2); // //
        let is_doc = self.peek() == Some(b'/');
        if is_doc {
            self.advance();
        }

        let start = self.current_pos;
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
                self.current_pos,
            ))))
        } else {
            Ok(Token::Comment(Comment::LineComment))
        }
    }

    fn read_block_comment(&mut self) -> Result<Token, TokenizeErr> {
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
        Err(TokenizeErr::BlockCommentNotClosed(start_err))
    }

    fn read_operator_or_delimiter(&mut self) -> Result<Token, TokenizeErr> {
        scan_operator_or_delimiter(self)
    }
}
