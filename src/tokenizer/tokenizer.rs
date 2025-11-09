use std::fmt::Debug;

use crate::tokenizer::Literal;
use super::{Comment, Delimiter, Keyword, Operator, Token};
use super::errors::TokenizeErr;

pub type Return<T> = Result<T, TokenizeErr>;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    tokens: Vec<Token<'a>>,
    current_char: usize,
    input: &'a str,
}

impl<'a> Tokenizer<'a> {
    /// Create a new tokenizer
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        Self {
            tokens: Vec::new(),
            current_char: 0,
            input,
        }
    }

    /// Tokenize the input string
    /// Returns a vector of tokens
    ///
    /// ```
    /// let mut tokenizer = Tokenizer::new("let x = 10;");
    /// let tokens = tokenizer.tokenize();
    /// assert_eq!(tokens.len(), 5);
    /// ```
    pub fn tokenize(mut self) -> Return<Vec<Token<'a>>> {
        let mut previous_index;

        loop {
            previous_index = self.current_char;
            if self
                .input
                .get(self.current_char..self.current_char + 1)
                .is_none()
            {
                break;
            }
            if self.skip_whitespace() {
                continue;
            } else if self.check_keyword() {
                continue;
            } else if self.check_comment()? {
                continue;
            } else if self.check_delimiter() {
                continue;
            } else if self.check_operator() {
                continue;
            } else if self.check_literal()? {
                continue;
            } else if self.check_identifier() {
                continue;
            }
            if previous_index == self.current_char {
                return Err(TokenizeErr::UnknownToken(previous_index));
            }
        }
        self.tokens.push(Token::EndOfFile);
        Ok(self.tokens)
    }

    #[inline]
    fn next(&mut self, target: &str) -> bool {
        if self.input[self.current_char..].starts_with(target) {
            self.current_char += target.len();
            true
        } else {
            false
        }
    }

    fn next_token(&mut self, target: &str) -> bool {
        if self.next(target) {
            if let Some(' ' | '\t' | '\n') = self.peek_char().unwrap_or(" ").chars().next() {
                self.current_char += 1;
                return true;
            }
            self.current_char -= target.len();
        }
        return false;
    }

    fn peek_char(&mut self) -> Option<&str> {
        self.input.get(self.current_char..self.current_char + 1)
    }

    fn skip_whitespace(&mut self) -> bool {
        let starts_index = self.current_char;
        while self.next(" ") || self.next("\n") || self.next("\t") {}
        self.current_char != starts_index
    }

    fn check_keyword(&mut self) -> bool {
        let keyword = if self.next_token("import") {
            Keyword::Import
        } else if self.next_token("from") {
            Keyword::From
        } else if self.next_token("fn") {
            Keyword::Fn
        } else if self.next_token("const") {
            Keyword::Const
        } else if self.next_token("let") {
            Keyword::Let
        } else if self.next_token("try") {
            Keyword::Try
        } else if self.next_token("class") {
            Keyword::Class
        } else if self.next_token("return") {
            Keyword::Return
        } else if self.next_token("pub") {
            Keyword::Pub
        } else if self.next_token("static") {
            Keyword::Static
        } else if self.next_token("final") {
            Keyword::Final
        } else if self.next_token("if") {
            Keyword::If
        } else if self.next_token("else") {
            Keyword::Else
        } else if self.next_token("for") {
            Keyword::For
        } else if self.next_token("in") {
            Keyword::In
        } else if self.next_token("while") {
            Keyword::While
        } else if self.next_token("break") {
            Keyword::Break
        } else if self.next_token("continue") {
            Keyword::Continue
        } else if self.next_token("match") {
            Keyword::Match
        } else if self.next_token("protocol") {
            Keyword::Protocol
        } else {
            return false;
        };

        self.tokens.push(Token::Keyword(keyword));
        true
    }

    fn check_literal(&mut self) -> Return<bool> {
        if let Some("\"") = self.peek_char() {
            self.current_char += 1;
            let starts_index = self.current_char;
            loop {
                if let Some("\"") = self.peek_char() {
                    break;
                }
                if self.peek_char().is_none() {
                    return Err(TokenizeErr::StringLiteralNotClosed(starts_index));
                }
                self.current_char += 1;
                if let Some("\n") = self.peek_char() {
                    return Err(TokenizeErr::StringLiteralNotClosed(starts_index));
                }
            }
            self.tokens.push(Token::Literal(Literal::StringLiteral(
                &self.input[starts_index..self.current_char],
            )));
            self.current_char += 1;
            Ok(true)
        } else if let Some("'") = self.peek_char() {
            self.current_char += 1;
            let starts_index = self.current_char;
            self.current_char += 1;
            loop {
                if let Some("'") = self.peek_char() {
                    break;
                }
                if self.peek_char().is_none() {
                    return Err(TokenizeErr::CharLiteralNotClosed(starts_index));
                }
                self.current_char += 1;
                if let Some("\n") = self.peek_char() {
                    return Err(TokenizeErr::CharLiteralNotClosed(starts_index));
                }
            }
            let c = &self.input[starts_index..self.current_char];
            if c.chars().count() != 1 {
                return Err(TokenizeErr::InvalidCharLiteral(starts_index));
            }
            self.tokens.push(Token::Literal(Literal::CharLiteral(
                c.chars().next().unwrap(),
            )));
            self.current_char += 1;
            Ok(true)
        } else if self
            .peek_char()
            .map(|c| c.chars().next().map(|c| c.is_digit(10)).unwrap_or(false))
            .unwrap_or(false)
        {
            let starts_index = self.current_char;
            loop {
                let next = self.peek_char();
                if next.is_none() {
                    break;
                }
                let next = next.unwrap().chars().next();
                if next.is_none() {
                    break;
                } else if next.unwrap().is_digit(10) {
                    self.current_char += 1;
                } else if next.unwrap() == '.' {
                    self.current_char += 1;
                    if let Some(".") = self.peek_char() {
                        self.current_char -= 1;
                        break;
                    }
                } else {
                    break;
                }
            }
            let num_string = &self.input[starts_index..self.current_char];
            if num_string.contains('.') {
                let float = num_string.parse::<f64>();
                let float64 = float
                    .clone()
                    .map(|float| float as f32)
                    .map(|float| float as f64);
                if float.is_err() {
                    return Err(TokenizeErr::InvalidFloatLiteral(self.current_char));
                }
                if float != float64 {
                    let double = num_string.parse::<f64>();
                    if double.is_err() {
                        return Err(TokenizeErr::InvalidFloatLiteral(self.current_char));
                    }
                    self.tokens
                        .push(Token::Literal(Literal::DoubleFloatLiteral(double.unwrap())));
                    return Ok(true);
                }
                self.tokens
                    .push(Token::Literal(Literal::FloatLiteral(float.unwrap() as f32)));
                Ok(true)
            } else {
                let int = num_string.parse::<i64>();
                let int64 = int.clone().map(|int| int as i32).map(|int| int as i64);
                if int.is_err() {
                    return Err(TokenizeErr::InvalidIntegerLiteral(self.current_char));
                }
                if int != int64 {
                    let double = num_string.parse::<i64>();
                    if double.is_err() {
                        return Err(TokenizeErr::InvalidIntegerLiteral(self.current_char));
                    }
                    self.tokens
                        .push(Token::Literal(Literal::DoubleIntegerLiteral(
                            double.unwrap(),
                        )));
                    return Ok(true);
                }
                self.tokens
                    .push(Token::Literal(Literal::IntegerLiteral(int.unwrap() as i32)));
                Ok(true)
            }
        } else if self.next("true") {
            self.tokens.push(Token::Literal(Literal::BoolLiteral(true)));
            Ok(true)
        } else if self.next("false") {
            self.tokens
                .push(Token::Literal(Literal::BoolLiteral(false)));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn check_operator(&mut self) -> bool {
        let operator = if self.next("**=") {
            Operator::PowAssign
        } else if self.next("+=") {
            Operator::AddAssign
        } else if self.next("-=") {
            Operator::SubAssign
        } else if self.next("*=") {
            Operator::MulAssign
        } else if self.next("/=") {
            Operator::DivAssign
        } else if self.next("%=") {
            Operator::ModAssign
        } else if self.next("==") {
            Operator::Equal
        } else if self.next("!=") {
            Operator::NotEqual
        } else if self.next(">=") {
            Operator::GreaterEqual
        } else if self.next("<=") {
            Operator::LessEqual
        } else if self.next("&&") {
            Operator::And
        } else if self.next("||") {
            Operator::Or
        } else if self.next("++") {
            Operator::Increment
        } else if self.next("--") {
            Operator::Decrement
        } else if self.next("**") {
            Operator::Pow
        } else if self.next("..") {
            Operator::CommaComma
        } else if self.next("+") {
            Operator::Plus
        } else if self.next("-") {
            Operator::Minus
        } else if self.next("*") {
            Operator::Multiply
        } else if self.next("/") {
            Operator::Divide
        } else if self.next("%") {
            Operator::Modulo
        } else if self.next(">") {
            Operator::Greater
        } else if self.next("<") {
            Operator::Less
        } else if self.next("!") {
            Operator::Not
        } else if self.next("=") {
            Operator::Assign
        } else {
            return false;
        };

        self.tokens.push(Token::Operator(operator));
        true
    }

    fn check_delimiter(&mut self) -> bool {
        let deleimiter = if self.next(",") {
            Delimiter::Comma
        } else if self.next(".") {
            Delimiter::Dot
        } else if self.next("::") {
            Delimiter::ColonColon
        } else if self.next(":") {
            Delimiter::Colon
        } else if self.next(";") {
            Delimiter::Semicolon
        } else if self.next("(") {
            Delimiter::LeftParen
        } else if self.next(")") {
            Delimiter::RightParen
        } else if self.next("{") {
            Delimiter::LeftBrace
        } else if self.next("}") {
            Delimiter::RightBrace
        } else if self.next("[") {
            Delimiter::LeftBracket
        } else if self.next("]") {
            Delimiter::RightBracket
        } else if self.next("@") {
            Delimiter::At
        } else if self.next("\\") {
            Delimiter::Backslash
        } else {
            return false;
        };

        self.tokens.push(Token::Delimiter(deleimiter));
        true
    }

    fn check_identifier(&mut self) -> bool {
        let starts_index = self.current_char;
        let first_char = self.peek_char();
        if first_char.is_none() {
            return false;
        }
        let first_char = first_char.unwrap().chars().next();
        if let Some('a'..='z' | 'A'..='Z' | '_') = first_char {
            self.current_char += 1;
        } else {
            return false;
        }
        loop {
            let next = self.peek_char();
            if next.is_none() {
                break;
            }
            let next = next.unwrap().chars().next();
            if next.is_none() {
                break;
            } else if let Some('a'..='z' | 'A'..='Z' | '_' | '0'..='9') = next {
                self.current_char += 1;
            } else {
                break;
            }
        }
        self.tokens.push(Token::Identifier(
            &self.input[starts_index..self.current_char],
        ));
        true
    }

    fn check_comment(&mut self) -> Return<bool> {
        if self.next("///") {
            let starts_index = self.current_char;
            loop {
                if let Some("\n") = self.peek_char() {
                    break;
                }
                if self.peek_char().is_none() {
                    break;
                }
                self.current_char += 1;
            }
            self.tokens.push(Token::Comment(Comment::DocComment(
                &self.input[starts_index..self.current_char],
            )));
            return Ok(true);
        } else if self.next("//") {
            loop {
                if let Some("\n") = self.peek_char() {
                    break;
                }
                if self.peek_char().is_none() {
                    break;
                }
                self.current_char += 1;
            }
            self.tokens.push(Token::Comment(Comment::LineComment));
            return Ok(true);
        } else if self.next("/*") {
            while !self.next("*/") {
                self.current_char += 1;
                if self.peek_char().is_none() {
                    return Err(TokenizeErr::BlockCommentNotClosed(self.current_char));
                }
            }
            self.tokens.push(Token::Comment(Comment::BlockComment));
            return Ok(true);
        }

        Ok(false)
    }
}
