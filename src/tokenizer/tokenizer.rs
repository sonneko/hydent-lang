use super::errors::TokenizeErr;
use super::{Comment, Delimiter, Keyword, Operator, Token};
use crate::common::span::Span;
use crate::tokenizer::Literal;
use crate::common::symbol::SymbolFactory;

pub type Return<T> = Result<T, TokenizeErr>;

pub struct Tokenizer<'a, 'sym> {
    tokens: Vec<Token>,
    current_char: usize,
    input: &'a str,
    symbol_factory: &'sym SymbolFactory<'sym>,
}

impl<'a, 'sym> Tokenizer<'a, 'sym> {
    /// Create a new tokenizer
    pub fn new(input: &'a str, symbol_factory: &mut SymbolFactory) -> Tokenizer<'a, 'sym> {
        Self {
            tokens: Vec::new(),
            current_char: 0,
            input,
            symbol_factory,
        }
    }

    /// Tokenize the input string
    /// Returns a vector of tokens
    ///
    /// ```
    /// use app::Tokenizer;
    /// let mut tokenizer = Tokenizer::new("let x = 10;");
    /// let tokens = tokenizer.tokenize();
    /// assert_eq!(tokens.unwrap().len(), 6);
    /// ```
    #[rustfmt::skip]
    pub fn tokenize(mut self) -> Return<Vec<Token>> {
        loop {
            if self
                .input
                .get(self.current_char..self.current_char + 1)
                .is_none()
            {
                break;
            }
            if self.skip_whitespace() { continue; } else
            if self.check_keyword() { continue; } else
            if self.check_comment()? { continue; } else
            if self.check_operator() { continue; } else
            if self.check_delimiter() { continue; } else
            if self.check_literal()? { continue; } else
            if self.check_identifier() { continue; }
            else {
                return Err(TokenizeErr::UnknownToken(self.current_char));
            }
        }
        self.tokens.push(Token::EndOfFile);
        Ok(self.tokens)
    }

    #[inline]
    fn next(&mut self, target: &str) -> bool {
        if self
            .input
            .get(self.current_char..)
            .map_or(false, |s| s.starts_with(target))
        {
            self.current_char += target.len();
            true
        } else {
            false
        }
    }

    #[inline]
    fn next_token(&mut self, target: &str) -> bool {
        if self.next(target) {
            if let Some(target) = self.peek_char() {
                let target = target;
                if !(target.is_alphanumeric() || target == '_') {
                    return true;
                }
            } else {
                return true;
            }
            self.current_char -= target.len();
        }
        return false;
    }

    #[inline]
    fn peek_char(&mut self) -> Option<char> {
        self.input
            .get(self.current_char..)
            .and_then(|s| s.chars().next())
    }

    fn skip_whitespace(&mut self) -> bool {
        let starts_index = self.current_char;
        while let Some(next_char) = self.peek_char() {
            if next_char.is_whitespace() {
                self.current_char += next_char.len_utf8();
            } else {
                break;
            }
        }
        self.current_char != starts_index
    }

    #[rustfmt::skip]
    fn check_keyword(&mut self) -> bool {
        let keyword = 
        if self.next_token("DoubleFloat") { Keyword::DoubleFloat } else
        if self.next_token("DoubleInt") { Keyword::DoubleInt } else
        if self.next_token("protocol") { Keyword::Protocol } else
        if self.next_token("continue") { Keyword::Continue } else
        if self.next_token("import") { Keyword::Import } else
        if self.next_token("static") { Keyword::Static } else
        if self.next_token("struct") { Keyword::Struct } else
        if self.next_token("extern") { Keyword::Extern } else
        if self.next_token("panics") { Keyword::Panics } else
        if self.next_token("module") { Keyword::Module } else
        if self.next_token("return") { Keyword::Return } else
        if self.next_token("ignore") { Keyword::Ignore } else
        if self.next_token("typeof") { Keyword::Typeof } else
        if self.next_token("class") { Keyword::Class } else
        if self.next_token("async") { Keyword::Async } else
        if self.next_token("match") { Keyword::Match } else
        if self.next_token("while") { Keyword::While } else
        if self.next_token("await") { Keyword::Await } else
        if self.next_token("break") { Keyword::Break } else
        if self.next_token("const") { Keyword::Const } else
        if self.next_token("final") { Keyword::Final } else
        if self.next_token("Float") { Keyword::Float } else
        if self.next_token("Usize") { Keyword::Usize } else
        if self.next_token("Never") { Keyword::Never } else
        if self.next_token("from") { Keyword::From } else
        if self.next_token("enum") { Keyword::Enum } else
        if self.next_token("type") { Keyword::Type } else
        if self.next_token("else") { Keyword::Else } else
        if self.next_token("loop") { Keyword::Loop } else
        if self.next_token("pipe") { Keyword::Pipe } else
        if self.next_token("this") { Keyword::This } else
        if self.next_token("impl") { Keyword::Impl } else
        if self.next_token("Bool") { Keyword::Bool } else
        if self.next_token("Char") { Keyword::Char } else
        if self.next_token("Void") { Keyword::Void } else
        if self.next_token("for") { Keyword::For } else
        if self.next_token("let") { Keyword::Let } else
        if self.next_token("try") { Keyword::Try } else
        if self.next_token("mut") { Keyword::Mut } else
        if self.next_token("pub") { Keyword::Pub } else
        if self.next_token("Int") { Keyword::Int } else
        if self.next_token("Any") { Keyword::Any } else
        if self.next_token("as") { Keyword::As } else
        if self.next_token("fn") { Keyword::Fn } else
        if self.next_token("if") { Keyword::If } else
        if self.next_token("in") { Keyword::In }
        else {
            return false;
        };
        self.tokens.push(Token::Keyword(keyword));
        true
    }

    fn check_literal(&mut self) -> Return<bool> {
        if let Some('"') = self.peek_char() {
            self.current_char += 1;
            let starts_index = self.current_char;
            loop {
                if let Some('"') = self.peek_char() {
                    break;
                }
                if self.peek_char().is_none() {
                    return Err(TokenizeErr::StringLiteralNotClosed(starts_index));
                }
                if let Some('\n') = self.peek_char() {
                    return Err(TokenizeErr::StringLiteralNotClosed(starts_index));
                }
                self.current_char += self.peek_char().unwrap().len_utf8();
            }
            self.tokens.push(Token::Literal(Literal::StringLiteral(
                Span::new(starts_index, self.current_char)
            )));
            self.current_char += 1;
            Ok(true)
        } else if let Some('\'') = self.peek_char() {
            self.current_char += 1;
            let starts_index = self.current_char;
            loop {
                if let Some('\'') = self.peek_char() {
                    break;
                } else if self.peek_char().is_none() {
                    return Err(TokenizeErr::CharLiteralNotClosed(starts_index));
                } else if let Some('\n') = self.peek_char() {
                    return Err(TokenizeErr::CharLiteralNotClosed(starts_index));
                } else if let Some('\\') = self.peek_char() {
                    self.current_char += 1;
                    if self.peek_char().is_none() {
                        return Err(TokenizeErr::CharLiteralNotClosed(starts_index));
                    }
                    self.current_char += self.peek_char().unwrap().len_utf8();
                    if Some('\\') == self.peek_char() {
                        if let Some('\'') = self.peek_char() {
                            self.current_char += 1;
                            self.tokens.push(Token::Literal(Literal::CharLiteral(
                                self.input[starts_index + 1..self.current_char - 1]
                                    .chars()
                                    .next()
                                    .unwrap(),
                            )));
                            return Ok(true);
                        } else {
                            return Err(TokenizeErr::CharLiteralNotClosed(starts_index));
                        }
                    }
                }
                self.current_char += self.peek_char().unwrap().len_utf8();
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
        } else if self.peek_char().map(|c| c.is_digit(10)).unwrap_or(false) {
            let starts_index = self.current_char;
            loop {
                let next = self.peek_char();
                if next.is_none() {
                    break;
                }
                let next = next;
                if next.is_none() {
                    break;
                } else if next.unwrap().is_digit(10) {
                    self.current_char += 1;
                } else if next.unwrap() == '.' {
                    self.current_char += 1;
                    if let Some('.') = self.peek_char() {
                        self.current_char -= 1;
                        break;
                    }
                } else {
                    break;
                }
            }
            let num_string = &self.input[starts_index..self.current_char];
            if num_string.contains('.') {
                // TOOD: implement DoubleFloat checking logic
                let float = num_string.parse::<f64>();
                if float.is_err() {
                    return Err(TokenizeErr::InvalidFloatLiteral(self.current_char));
                }
                self.tokens
                    .push(Token::Literal(Literal::FloatLiteral(float.unwrap() as f32)));
                Ok(true)
            } else {
                // TODO: implement DoubleInt checking logic
                let int = num_string.parse::<i64>();
                if int.is_err() {
                    return Err(TokenizeErr::InvalidIntegerLiteral(self.current_char));
                }
                self.tokens
                    .push(Token::Literal(Literal::IntegerLiteral(int.unwrap() as i32)));
                Ok(true)
            }
        } else if self.next_token("true") {
            self.tokens.push(Token::Literal(Literal::BoolLiteral(true)));
            Ok(true)
        } else if self.next_token("false") {
            self.tokens
                .push(Token::Literal(Literal::BoolLiteral(false)));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[rustfmt::skip]
    fn check_operator(&mut self) -> bool {
        let operator = 
        if self.next("..=") { Operator::RangeInclusive     } else
        if self.next("=>") { Operator::FatArrow           } else
        if self.next("|>") { Operator::Pipe               } else
        if self.next("->") { Operator::Arrow              } else
        if self.next("::") { Operator::NamespaceResolver  } else
        if self.next("||") { Operator::LogicalOr          } else
        if self.next("&&") { Operator::LogicalAnd         } else
        if self.next("==") { Operator::Equality           } else
        if self.next("!=") { Operator::Inequality         } else
        if self.next("<=") { Operator::LessThanOrEqual    } else
        if self.next(">=") { Operator::GreaterThanOrEqual } else
        if self.next("<<") { Operator::ShiftLeft          } else
        if self.next(">>") { Operator::ShiftRight         } else
        if self.next("**") { Operator::PowerOf            } else
        if self.next("..") { Operator::RangeExclusive     } else
        if self.next("+=") { Operator::AddAssign          } else
        if self.next("-=") { Operator::SubtractAssign     } else
        if self.next("*=") { Operator::MultiplyAssign     } else
        if self.next("/=") { Operator::DivideAssign       } else
        if self.next("*") { Operator::Multiply           } else
        if self.next("=") { Operator::Assignment         } else
        if self.next(":") { Operator::Colon              } else
        if self.next("@") { Operator::At                 } else
        if self.next("|") { Operator::Or                 } else
        if self.next("^") { Operator::Xor                } else
        if self.next("&") { Operator::And                } else
        if self.next("<") { Operator::LessThan           } else
        if self.next(">") { Operator::GreaterThan        } else
        if self.next("+") { Operator::Add                } else
        if self.next("-") { Operator::Subtract           } else
        if self.next("/") { Operator::Divide             } else
        if self.next("%") { Operator::Remainder          } else
        if self.next("!") { Operator::Not                } else
        if self.next("~") { Operator::BitwiseNot         } else
        if self.next(".") { Operator::MemberAccess       } else
        if self.next("_") { Operator::Wildcard           } else {
            return false;
        };
        self.tokens.push(Token::Operator(operator));
        true
    }

    #[rustfmt::skip]
    fn check_delimiter(&mut self) -> bool {
        let deleimiter = 
        if self.next(";") {Delimiter::Semicolon    } else
        if self.next("{") {Delimiter::LeftBrace    } else
        if self.next("}") {Delimiter::RightBrace   } else
        if self.next("(") {Delimiter::LeftParen    } else
        if self.next(")") {Delimiter::RightParen   } else
        if self.next(",") {Delimiter::Comma        } else
        if self.next("[") {Delimiter::LeftBracket  } else
        if self.next("]") {Delimiter::RightBracket }
        else {
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
        if let Some('a'..='z' | 'A'..='Z' | '_') = first_char {
            self.current_char += 1;
        } else {
            return false;
        }
        loop {
            let next = self.peek_char();
            if next.is_none() {
                break;
            } else if let Some('a'..='z' | 'A'..='Z' | '_' | '0'..='9') = next {
                self.current_char += 1;
            } else {
                break;
            }
        }
        self.tokens.push(Token::Identifier(
            self.symbol_factory.from_range(starts_index, self.current_char)
        ));
        true
    }

    fn check_comment(&mut self) -> Return<bool> {
        if self.next("///") {
            let starts_index = self.current_char;
            loop {
                if let Some('\n') = self.peek_char() {
                    break;
                }
                if self.peek_char().is_none() {
                    break;
                }
                self.current_char += self.peek_char().unwrap().len_utf8();
            }
            self.tokens.push(Token::Comment(Comment::DocComment(
                Span::new(starts_index, self.current_char)
            )));
            return Ok(true);
        } else if self.next("//") {
            loop {
                if let Some('\n') = self.peek_char() {
                    self.next("\n");
                    break;
                }
                if self.peek_char().is_none() {
                    break;
                }
                self.current_char += self.peek_char().unwrap().len_utf8();
            }
            self.tokens.push(Token::Comment(Comment::LineComment));
            return Ok(true);
        } else if self.next("/*") {
            while !self.next("*/") {
                if self.peek_char().is_none() {
                    return Err(TokenizeErr::BlockCommentNotClosed(self.current_char));
                }
                self.current_char += self.peek_char().unwrap().len_utf8();
            }
            self.tokens.push(Token::Comment(Comment::BlockComment));
            return Ok(true);
        }

        Ok(false)
    }
}
