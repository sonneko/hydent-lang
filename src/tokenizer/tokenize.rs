//! Tokenizer with a function to intern strings

use crate::compiler::span::Span;
use crate::compiler::symbol::SymbolFactory;
use crate::tokenizer::errors::TokenizeErr;
use crate::tokenizer::tokens::{Comment, Delimiter, Keyword, Literal, Operator, Token};

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

    pub fn tokenize(mut self) -> (Vec<(Token, Span)>, Vec<(TokenizeErr, Span)>) {
        let mut tokens = Vec::with_capacity(self.input.len() / 4); // expect we need length/4 vector
        let mut errors = Vec::new();

        while let Some(b) = self.peek() {
            let begin = self.current_pos;
            let next = match b {
                b' ' | b'\t' | b'\r' | b'\n' => {
                    self.advance();
                    continue;
                }
                b'/' => {
                    if let Some(next) = self.peek_at(1) {
                        match next {
                            b'/' => self.read_line_comment(),
                            b'*' => self.read_block_comment(),
                            b'=' => {
                                self.advance_n(2);
                                Ok(Token::Operator(Operator::DivideAssign))
                            }
                            _ => {
                                self.advance();
                                Ok(Token::Operator(Operator::Divide))
                            }
                        }
                    } else {
                        self.advance();
                        Ok(Token::Operator(Operator::Divide))
                    }
                }
                b'"' => self.read_string_literal(),
                b'\'' => self.read_char_literal(),
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => Ok(self.read_identifier_or_keyword()),
                b'0'..=b'9' => self.read_number_literal(),
                b'.' => {
                    if self.consume_str(b"..=") {
                        Ok(Token::Operator(Operator::RangeInclusive))
                    } else if self.consume_str(b"..") {
                        Ok(Token::Operator(Operator::RangeExclusive))
                    } else {
                        self.advance();
                        Ok(Token::Operator(Operator::MemberAccess))
                    }
                }
                _ => {
                    if let Some(op) = self.read_operator_or_delimiter() {
                        Ok(op)
                    } else {
                        Err(TokenizeErr::UnknownToken(self.current_pos))
                    }
                }
            };

            match next {
                Ok(token) => tokens.push((token, Span::new(begin, self.current_pos))),
                Err(err) => {
                    errors.push((err, Span::new(begin, self.current_pos)));
                    tokens.push((Token::Invalid, Span::new(begin, self.current_pos)));
                },
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

    // --- logic to read each tokens ---

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

        // expect rustc to replace this with much faster logic such as phf
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
                // not keywords
                let symbol = self.symbol_factory.from_range(start, self.current_pos);
                Token::Identifier(symbol)
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

        // SAFETY: `self.input` is a `&[u8]`, and `start` and `self.current_pos` are valid
        // indices within `self.input` that have been advanced based on valid UTF-8
        // characters or ASCII digits/hex digits/binary digits,
        // or other ASCII characters.
        // All these are valid UTF-8, so `from_utf8_unchecked` is safe.
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

    fn read_operator_or_delimiter(&mut self) -> Option<Token> {
        let b = self.peek()?;

        // TODO: make this logic faster
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
