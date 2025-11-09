use crate::tokenizer::Literal;

use super::{Keyword, Operator, Token};

pub enum TokenizeErr {
    StringLiteralNotClosed,
    CharLiteralNotClosed,
    InvalidIntegerLiteral,
    InvalidFloatLiteral,
}

type Return<T> = Result<T, TokenizeErr>;

pub struct Tokenizer<'a> {
    tokens: Vec<Token<'a>>,
    current_char: usize,
    input: String,
}

impl<'a> Tokenizer<'a> {
    /// Create a new tokenizer
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current_char: 0,
            input: String::new(),
        }
    }

    /// Tokenize the input string
    /// Returns a vector of tokens
    ///
    /// ```
    /// let mut tokenizer = Tokenizer::new();
    /// let tokens = tokenizer.tokenize("let x = 10;");
    /// assert_eq!(tokens.len(), 5);
    /// ```
    pub fn tokenize(&'a mut self, input: &str) -> Return<Vec<Token>> {
        self.input = input.into();
        self.skip_whitespace();
        self.check_keyword();
        self.check_literal()?;

        unimplemented!()
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

    fn peek_char(&mut self) -> &str {
        &self.input[self.current_char..self.current_char + 1]
    }

    fn skip_whitespace(&mut self) {
        while self.next(" ") || self.next("\n") || self.next("\t") {}
    }

    fn check_keyword(&mut self) {
        let keyword = if self.next("import") {
            Keyword::Import
        } else if self.next("from") {
            Keyword::From
        } else if self.next("fn") {
            Keyword::Fn
        } else if self.next("const") {
            Keyword::Const
        } else if self.next("let") {
            Keyword::Let
        } else if self.next("try") {
            Keyword::Try
        } else if self.next("class") {
            Keyword::Class
        } else if self.next("return") {
            Keyword::Return
        } else if self.next("pub") {
            Keyword::Pub
        } else if self.next("static") {
            Keyword::Static
        } else if self.next("final") {
            Keyword::Final
        } else if self.next("if") {
            Keyword::If
        } else if self.next("for") {
            Keyword::For
        } else if self.next("in") {
            Keyword::In
        } else if self.next("while") {
            Keyword::While
        } else if self.next("break") {
            Keyword::Break
        } else if self.next("continue") {
            Keyword::Continue
        } else if self.next("match") {
            Keyword::Match
        } else if self.next("protocol") {
            Keyword::Protocol
        } else {
            return;
        };

        self.tokens.push(Token::Keyword(keyword));
    }

    fn check_literal(&'a mut self) -> Return<()> {
        if self.peek_char() == "\"" {
            let starts_index = self.current_char;
            while self.peek_char() != "\"" {
                self.current_char += 1;
                if self.peek_char() == "\n" {
                    return Err(TokenizeErr::StringLiteralNotClosed);
                }
            }
            self.tokens.push(Token::Literal(Literal::StringLiteral(
                &self.input[starts_index..self.current_char - 1],
            )))
        } else if self.peek_char() == "\'" {
            self.current_char += 1;
            let c = self.peek_char().chars().next();
            self.current_char += 1;
            if c.is_none() || self.peek_char() != "\'" {
                return Err(TokenizeErr::CharLiteralNotClosed);
            }
            self.current_char += 1;
            self.tokens
                .push(Token::Literal(Literal::CharLiteral(c.unwrap())));
        } else if self
            .peek_char()
            .chars()
            .next()
            .map(|c| c.is_digit(10))
            .unwrap_or(false)
        {
            let starts_index = self.current_char;
            loop {
                let next = self.peek_char().chars().next();
                if next.is_none() {
                    break;
                } else if next.unwrap().is_digit(10) {
                    self.current_char += 1;
                } else {
                    break;
                }
            }
            let num_string = &self.input[starts_index..self.current_char];
            if num_string.contains('.') {
                let float = num_string.parse::<f32>();
                if float.is_err() {
                    let double = num_string.parse::<f64>();
                    if double.is_err() {
                        return Err(TokenizeErr::InvalidFloatLiteral);
                    }
                    self.tokens
                        .push(Token::Literal(Literal::DoubleFloatLiteral(double.unwrap())));
                }
                self.tokens
                    .push(Token::Literal(Literal::FloatLiteral(float.unwrap())));
            } else {
                let int = num_string.parse::<i32>();
                if int.is_err() {
                    let double = num_string.parse::<i64>();
                    if double.is_err() {
                        return Err(TokenizeErr::InvalidIntegerLiteral);
                    }
                    self.tokens
                        .push(Token::Literal(Literal::DoubleIntegerLiteral(
                            double.unwrap(),
                        )));
                }
                self.tokens
                    .push(Token::Literal(Literal::IntegerLiteral(int.unwrap())));
            }
        } else if self.next("true") {
            self.tokens.push(Token::Literal(Literal::BoolLiteral(true)));
        } else if self.next("false") {
            self.tokens
                .push(Token::Literal(Literal::BoolLiteral(false)));
        }

        Ok(())
    }

    fn check_operator(&mut self) {
        let operator = if self.next("+") {
            Operator::Plus
        } else if self.next("-") {
            Operator::Minus
        } else if self.next("*") {
            Operator::Multiply
        } else if self.next("**") {
            Operator::Pow
        } else if self.next("/") {
            Operator::Divide
        } else if self.next("%") {
            Operator::Modulo
        } else if self.next("+=") {
            Operator::AddAssign
        } else if self.next("-=") {
            Operator::SubAssign
        } else if self.next("*=") {
            Operator::MulAssign
        } else if self.next("**=") {
            Operator::PowAssign
        } else if self.next("/=") {
            Operator::DivAssign
        } else if self.next("%=") {
            Operator::ModAssign
        } else if self.next("==") {
            Operator::Equal
        } else if self.next("!=") {
            Operator::NotEqual
        } else if self.next(">") {
            Operator::Greater
        } else if self.next("<") {
            Operator::Less
        } else if self.next(">=") {
            Operator::GreaterEqual
        } else if self.next("<=") {
            Operator::LessEqual
        } else if self.next("&&") {
            Operator::And
        } else if self.next("||") {
            Operator::Or
        } else if self.next("!") {
            Operator::Not
        } else if self.next("++") {
            Operator::Increment
        } else if self.next("--") {
            Operator::Decrement
        } else if self.next("=") {
            Operator::Assign
        } else {
            return;
        };

        self.tokens.push(Token::Operator(operator));
    }
}
