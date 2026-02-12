//! This module implements the tokenizer for the Hydent programming language.
//!
//! The `Tokenizer` is responsible for taking a raw source code string and
//! breaking it down into a stream of meaningful tokens, such as keywords,
//! identifiers, literals, operators, and delimiters. This is the first phase
//! of the compiler's frontend.

use super::errors::TokenizeErr;
use crate::compiler::span::Span;
use crate::compiler::symbol::SymbolFactory;
use crate::tokenizer::tokens::{Comment, Delimiter, Keyword, Literal, Operator, Token};

pub type Return<T> = Result<T, TokenizeErr>;

pub struct Tokenizer<'src, 'ctx> {
    current_pos: usize,
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

    pub fn tokenize(mut self) -> Return<Vec<Token>> {
        let mut tokens = Vec::with_capacity(self.input.len() / 4); // 投機的なアロケーション

        while let Some(b) = self.peek() {
            match b {
                b' ' | b'\t' | b'\r' | b'\n' => {
                    self.advance();
                }
                b'/' => {
                    if let Some(next) = self.peek_at(1) {
                        match next {
                            b'/' => {
                                tokens.push(self.read_line_comment()?);
                            }
                            b'*' => {
                                tokens.push(self.read_block_comment()?);
                            }
                            b'=' => {
                                self.advance_n(2);
                                tokens.push(Token::Operator(Operator::DivideAssign));
                            }
                            _ => {
                                self.advance();
                                tokens.push(Token::Operator(Operator::Divide));
                            }
                        }
                    } else {
                        self.advance();
                        tokens.push(Token::Operator(Operator::Divide));
                    }
                }
                b'"' => tokens.push(self.read_string_literal()?),
                b'\'' => tokens.push(self.read_char_literal()?),
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    tokens.push(self.read_identifier_or_keyword());
                }
                b'0'..=b'9' => {
                    tokens.push(self.read_number_literal()?);
                }
                b'.' => {
                    if self.consume_str(b"..=") {
                        tokens.push(Token::Operator(Operator::RangeInclusive));
                    } else if self.consume_str(b"..") {
                        tokens.push(Token::Operator(Operator::RangeExclusive));
                    } else {
                        self.advance();
                        tokens.push(Token::Operator(Operator::MemberAccess));
                    }
                }
                _ => {
                    if let Some(op) = self.read_operator_or_delimiter() {
                        tokens.push(op);
                    } else {
                        return Err(TokenizeErr::UnknownToken(self.current_pos));
                    }
                }
            }
        }

        tokens.push(Token::EndOfFile);
        Ok(tokens)
    }

    // --- 低レベルヘルパー ---

    #[inline(always)]
    fn peek(&self) -> Option<u8> {
        self.input.get(self.current_pos).copied()
    }

    #[inline(always)]
    fn peek_at(&self, offset: usize) -> Option<u8> {
        self.input.get(self.current_pos + offset).copied()
    }

    #[inline(always)]
    fn advance(&mut self) {
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

    // --- トークン読み取りロジック ---

    fn read_identifier_or_keyword(&mut self) -> Token {
        let start = self.current_pos;
        while let Some(b) = self.peek() {
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.advance();
            } else {
                break;
            }
        }
        let slice = &self.input[start..self.current_pos];

        // キーワード判定 (Matchはコンパイラが自動で高速なジャンプテーブル/分岐に最適化する)
        match slice {
            b"import" => Token::Keyword(Keyword::Import),
            b"from" => Token::Keyword(Keyword::From),
            b"static" => Token::Keyword(Keyword::Static),
            b"class" => Token::Keyword(Keyword::Class),
            b"enum" => Token::Keyword(Keyword::Enum),
            b"struct" => Token::Keyword(Keyword::Struct),
            b"fn" => Token::Keyword(Keyword::Fn),
            b"protocol" => Token::Keyword(Keyword::Protocol),
            b"module" => Token::Keyword(Keyword::Module),
            b"type" => Token::Keyword(Keyword::Type),
            b"pub" => Token::Keyword(Keyword::Pub),
            b"extern" => Token::Keyword(Keyword::Extern),
            b"async" => Token::Keyword(Keyword::Async),
            b"panics" => Token::Keyword(Keyword::Panics),
            b"mut" => Token::Keyword(Keyword::Mut),
            b"final" => Token::Keyword(Keyword::Final),
            b"as" => Token::Keyword(Keyword::As),
            b"try" => Token::Keyword(Keyword::Try),
            b"await" => Token::Keyword(Keyword::Await),
            b"this" => Token::Keyword(Keyword::This),
            b"ignore" => Token::Keyword(Keyword::Ignore),
            b"let" => Token::Keyword(Keyword::Let),
            b"const" => Token::Keyword(Keyword::Const),
            b"return" => Token::Keyword(Keyword::Return),
            b"break" => Token::Keyword(Keyword::Break),
            b"continue" => Token::Keyword(Keyword::Continue),
            b"if" => Token::Keyword(Keyword::If),
            b"else" => Token::Keyword(Keyword::Else),
            b"match" => Token::Keyword(Keyword::Match),
            b"loop" => Token::Keyword(Keyword::Loop),
            b"while" => Token::Keyword(Keyword::While),
            b"for" => Token::Keyword(Keyword::For),
            b"in" => Token::Keyword(Keyword::In),
            b"pipe" => Token::Keyword(Keyword::Pipe),
            b"impl" => Token::Keyword(Keyword::Impl),
            b"typeof" => Token::Keyword(Keyword::Typeof),
            b"Bool" => Token::Keyword(Keyword::Bool),
            b"Int" => Token::Keyword(Keyword::Int),
            b"DoubleInt" => Token::Keyword(Keyword::DoubleInt),
            b"Float" => Token::Keyword(Keyword::Float),
            b"DoubleFloat" => Token::Keyword(Keyword::DoubleFloat),
            b"Char" => Token::Keyword(Keyword::Char),
            b"Usize" => Token::Keyword(Keyword::Usize),
            b"Any" => Token::Keyword(Keyword::Any),
            b"Never" => Token::Keyword(Keyword::Never),
            b"Void" => Token::Keyword(Keyword::Void),
            _ => {
                // キーワードでなければSymbolInterning
                let symbol = self.symbol_factory.from_range(start, self.current_pos);
                Token::Identifier(symbol)
            }
        }
    }

    fn read_number_literal(&mut self) -> Return<Token> {
        let start = self.current_pos;
        let mut is_float = false;

        // 16進数等のプレフィックス対応
        if self.consume_str(b"0x") {
            while let Some(b) = self.peek() {
                if b.is_ascii_hexdigit() {
                    self.advance();
                } else {
                    break;
                }
            }
        } else if self.consume_str(b"0b") {
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
                        } // .. 演算子への配慮
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
        if is_float {
            slice
                .parse::<f32>()
                .map(|v| Token::Literal(Literal::FloatLiteral(v.into())))
                .map_err(|_| TokenizeErr::InvalidFloatLiteral(start))
        } else {
            slice
                .parse::<i32>()
                .map(|v| Token::Literal(Literal::IntegerLiteral(v.into())))
                .map_err(|_| TokenizeErr::InvalidIntegerLiteral(start))
        }
    }

    fn read_string_literal(&mut self) -> Return<Token> {
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

    fn read_char_literal(&mut self) -> Return<Token> {
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

    fn read_line_comment(&mut self) -> Return<Token> {
        self.advance_n(2); // //
        let is_doc = self.peek() == Some(b'/');
        if is_doc {
            self.advance();
        }

        let start = self.current_pos;
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

    fn read_block_comment(&mut self) -> Return<Token> {
        let start_err = self.current_pos;
        self.advance_n(2); // /*
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

    fn read_operator_or_delimiter(&mut self) -> Option<Token> {
        let b = self.peek()?;

        if self.consume_str(b"=>") {
            return Some(Token::Operator(Operator::FatArrow));
        }
        if self.consume_str(b"|>") {
            return Some(Token::Operator(Operator::Pipe));
        }
        if self.consume_str(b"->") {
            return Some(Token::Operator(Operator::Arrow));
        }
        if self.consume_str(b"::") {
            return Some(Token::Operator(Operator::NamespaceResolver));
        }
        if self.consume_str(b"||") {
            return Some(Token::Operator(Operator::LogicalOr));
        }
        if self.consume_str(b"&&") {
            return Some(Token::Operator(Operator::LogicalAnd));
        }
        if self.consume_str(b"==") {
            return Some(Token::Operator(Operator::Equality));
        }
        if self.consume_str(b"!=") {
            return Some(Token::Operator(Operator::Inequality));
        }
        if self.consume_str(b"<=") {
            return Some(Token::Operator(Operator::LessThanOrEqual));
        }
        if self.consume_str(b">=") {
            return Some(Token::Operator(Operator::GreaterThanOrEqual));
        }
        if self.consume_str(b"<<") {
            return Some(Token::Operator(Operator::ShiftLeft));
        }
        if self.consume_str(b">>") {
            return Some(Token::Operator(Operator::ShiftRight));
        }
        if self.consume_str(b"**") {
            return Some(Token::Operator(Operator::PowerOf));
        }
        if self.consume_str(b"+=") {
            return Some(Token::Operator(Operator::AddAssign));
        }
        if self.consume_str(b"-=") {
            return Some(Token::Operator(Operator::SubtractAssign));
        }
        if self.consume_str(b"*=") {
            return Some(Token::Operator(Operator::MultiplyAssign));
        }

        self.advance();
        match b {
            b';' => Some(Token::Delimiter(Delimiter::Semicolon)),
            b'{' => Some(Token::Delimiter(Delimiter::LeftBrace)),
            b'}' => Some(Token::Delimiter(Delimiter::RightBrace)),
            b'(' => Some(Token::Delimiter(Delimiter::LeftParen)),
            b')' => Some(Token::Delimiter(Delimiter::RightParen)),
            b'[' => Some(Token::Delimiter(Delimiter::LeftBracket)),
            b']' => Some(Token::Delimiter(Delimiter::RightBracket)),
            b',' => Some(Token::Delimiter(Delimiter::Comma)),
            b':' => Some(Token::Operator(Operator::Colon)),
            b'=' => Some(Token::Operator(Operator::Assignment)),
            b'*' => Some(Token::Operator(Operator::Multiply)),
            b'+' => Some(Token::Operator(Operator::Add)),
            b'-' => Some(Token::Operator(Operator::Subtract)),
            b'/' => Some(Token::Operator(Operator::Divide)),
            b'%' => Some(Token::Operator(Operator::Remainder)),
            b'<' => Some(Token::Operator(Operator::LessThan)),
            b'>' => Some(Token::Operator(Operator::GreaterThan)),
            b'&' => Some(Token::Operator(Operator::And)),
            b'|' => Some(Token::Operator(Operator::Or)),
            b'^' => Some(Token::Operator(Operator::Xor)),
            b'!' => Some(Token::Operator(Operator::Not)),
            b'~' => Some(Token::Operator(Operator::BitwiseNot)),
            b'@' => Some(Token::Operator(Operator::At)),
            b'_' => Some(Token::Operator(Operator::Wildcard)),
            _ => None,
        }
    }
}
