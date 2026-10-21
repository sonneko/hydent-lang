use crate::compiler::span::Span;
use crate::diagnostic::stream::DiagnosticStream;
use crate::parser::base_parser::BaseParser;
use crate::parser::errors::{IParseErr, ParseErr};
use crate::parser::generated_ast;
use crate::parser::generated_parser::GeneratedParser;
use crate::parser::tracer::Tracer;
use crate::parser::Parser;
use crate::tokenizer::tokens::{Comment, Delimiter, Keyword, Literal, Operator, Token};

impl<S: DiagnosticStream, TR: Tracer> Parser<'_, '_, '_, S, TR> {
    fn take_raw_syntax_token(
        &mut self,
        depth: u32,
    ) -> Result<generated_ast::RawSyntaxToken, ParseErr> {
        let Some((token, span)) = self.tokens.peek(0) else {
            return Err(ParseErr::build(false, &[], self.enviroment()));
        };
        if token == Token::EndOfFile {
            return Err(ParseErr::build(false, &[], self.enviroment()));
        }

        self.tokens.next();
        Ok(generated_ast::RawSyntaxToken { token, span, depth })
    }

    fn raw_declaration_category(&self) -> &'static str {
        if self.peek::<0>() == Some(Token::Operator(Operator::At)) {
            return "Annotation";
        }

        for index in 0..16 {
            match self.tokens.peek(index).map(|(token, _)| token) {
                Some(Token::Keyword(Keyword::Import)) => return "ImportDeclaration",
                Some(Token::Keyword(Keyword::Module)) => return "Module",
                Some(Token::Keyword(Keyword::Fn)) => return "Function",
                Some(Token::Keyword(Keyword::Extern)) => return "ExternFunction",
                Some(Token::Keyword(Keyword::Struct)) => return "Struct",
                Some(Token::Keyword(Keyword::Class)) => return "Class",
                Some(Token::Keyword(Keyword::Protocol)) => return "Protocol",
                Some(Token::Keyword(Keyword::Enum)) => return "Enum",
                Some(Token::Keyword(Keyword::Type)) => return "TypeAlias",
                Some(Token::Keyword(Keyword::Const)) => return "Constant",
                Some(Token::Keyword(Keyword::Static)) => return "Static",
                Some(Token::EndOfFile) | None => break,
                _ => {}
            }
        }
        "Syntax"
    }
}

impl<S: DiagnosticStream, TR: Tracer> GeneratedParser for Parser<'_, '_, '_, S, TR> {
    type TraceGuard = TR::Guard;
    fn trace(name: &'static str) -> TR::Guard {
        TR::trace(name)
    }

    fn parse_RawSyntaxToken(&mut self) -> Result<generated_ast::RawSyntaxToken, Self::Error> {
        self.take_raw_syntax_token(0)
    }

    fn parse_RawDeclaration(&mut self) -> Result<generated_ast::RawDeclaration, Self::Error> {
        self.tokens.checkpoint();
        let start = self.now_span();
        let category = self.raw_declaration_category();
        let mut syntax = Vec::new();
        let mut brace_depth = 0u32;
        let mut paren_depth = 0u32;
        let mut bracket_depth = 0u32;
        let mut body_started = false;
        let mut body_ended = false;
        let mut semicolon_terminated = false;
        let mut saw_function_keyword = false;
        let mut expect_function_name = false;
        let mut function_name_seen = false;
        let mut function_parameters_seen = false;
        let mut invalid_closer = false;
        let mut invalid_token = false;
        let mut invalid_at = false;
        let mut end = start.end;

        loop {
            let Some((token, span)) = self.tokens.peek(0) else {
                break;
            };
            if token == Token::EndOfFile {
                break;
            }

            let at_top_level = brace_depth == 0 && paren_depth == 0 && bracket_depth == 0;
            let depth = brace_depth + paren_depth + bracket_depth;
            if at_top_level && token == Token::Delimiter(Delimiter::LeftBrace) {
                body_started = true;
            }
            if at_top_level && token == Token::Keyword(Keyword::Fn) {
                saw_function_keyword = true;
                expect_function_name = true;
            } else if expect_function_name {
                function_name_seen = matches!(token, Token::Identifier(_));
                expect_function_name = false;
            }
            if at_top_level
                && saw_function_keyword
                && token == Token::Delimiter(Delimiter::LeftParen)
            {
                function_parameters_seen = true;
            }
            if category == "Function" && token == Token::Operator(Operator::At) && depth > 0 {
                invalid_at = true;
            }
            if token == Token::Invalid {
                invalid_token = true;
            }

            syntax.push(self.take_raw_syntax_token(depth)?);
            end = span.end;

            match token {
                Token::Delimiter(Delimiter::LeftBrace) => brace_depth += 1,
                Token::Delimiter(Delimiter::RightBrace) => {
                    if brace_depth == 0 {
                        invalid_closer = true;
                    } else {
                        brace_depth -= 1;
                        if body_started
                            && brace_depth == 0
                            && paren_depth == 0
                            && bracket_depth == 0
                        {
                            body_ended = true;
                            break;
                        }
                    }
                }
                Token::Delimiter(Delimiter::LeftParen) => paren_depth += 1,
                Token::Delimiter(Delimiter::RightParen) => {
                    if paren_depth == 0 {
                        invalid_closer = true;
                    } else {
                        paren_depth -= 1;
                    }
                }
                Token::Delimiter(Delimiter::LeftBracket) => bracket_depth += 1,
                Token::Delimiter(Delimiter::RightBracket) => {
                    if bracket_depth == 0 {
                        invalid_closer = true;
                    } else {
                        bracket_depth -= 1;
                    }
                }
                Token::Delimiter(Delimiter::Semicolon)
                    if brace_depth == 0 && paren_depth == 0 && bracket_depth == 0 =>
                {
                    semicolon_terminated = true;
                    break;
                }
                _ => {}
            }
        }

        let is_function = category == "Function" || category == "ExternFunction";
        let function_is_valid = !is_function
            || (saw_function_keyword
                && function_name_seen
                && function_parameters_seen
                && (body_ended || semicolon_terminated));
        let delimiters_are_balanced = brace_depth == 0 && paren_depth == 0 && bracket_depth == 0;
        let has_invalid_function_syntax = category == "Function" && invalid_at;
        if syntax.is_empty()
            || !delimiters_are_balanced
            || invalid_closer
            || invalid_token
            || has_invalid_function_syntax
            || !function_is_valid
            || (body_started && !body_ended)
        {
            let error = ParseErr::build(false, &[], self.enviroment());
            self.tokens.rollback();
            return Err(error);
        }

        self.tokens.commit();
        let syntax = self.ctx.ast_arena.alloc_iter(syntax.into_iter());
        Ok(generated_ast::RawDeclaration {
            category,
            span: Span::new(start.begin as usize, end as usize),
            syntax,
        })
    }

    fn parse_Identifier(&mut self) -> Result<generated_ast::Identifier, Self::Error> {
        if let Some(Token::Identifier(symbol)) = self.peek::<0>() {
            self.consume_token();
            Ok(generated_ast::Identifier { symbol })
        } else {
            Err(Self::Error::build(true, &[], self.enviroment()))
        }
    }
    fn parse_StringLiteral(&mut self) -> Result<generated_ast::StringLiteral, Self::Error> {
        if let Some(Token::Literal(Literal::StringLiteral(span))) = self.peek::<0>() {
            self.consume_token();
            Ok(generated_ast::StringLiteral { span })
        } else {
            Err(Self::Error::build(false, &[], self.enviroment()))
        }
    }
    fn parse_DocComment(&mut self) -> Result<generated_ast::DocComment, Self::Error> {
        if let Some(Token::Comment(Comment::DocComment(span))) = self.peek::<0>() {
            self.consume_token();
            Ok(generated_ast::DocComment { span })
        } else {
            Err(Self::Error::build(false, &[], self.enviroment()))
        }
    }

    fn parse_CharLiteral(&mut self) -> Result<generated_ast::CharLiteral, Self::Error> {
        if let Some(Token::Literal(Literal::CharLiteral(value))) = self.peek::<0>() {
            self.consume_token();
            Ok(generated_ast::CharLiteral { value })
        } else {
            Err(Self::Error::build(false, &[], self.enviroment()))
        }
    }

    fn parse_IntLiteral(&mut self) -> Result<generated_ast::IntLiteral, Self::Error> {
        if let Some(Token::Literal(Literal::IntegerLiteral(value))) = self.peek::<0>() {
            self.consume_token();
            Ok(generated_ast::IntLiteral { value })
        } else {
            Err(Self::Error::build(false, &[], self.enviroment()))
        }
    }

    fn parse_FloatLiteral(&mut self) -> Result<generated_ast::FloatLiteral, Self::Error> {
        if let Some(Token::Literal(Literal::FloatLiteral(value))) = self.peek::<0>() {
            self.consume_token();
            Ok(generated_ast::FloatLiteral { value })
        } else {
            Err(Self::Error::build(false, &[], self.enviroment()))
        }
    }

    fn parse_BoolLiteral(&mut self) -> Result<generated_ast::BoolLiteral, Self::Error> {
        if let Some(Token::Literal(Literal::BoolLiteral(value))) = self.peek::<0>() {
            self.consume_token();
            Ok(generated_ast::BoolLiteral { value })
        } else {
            Err(Self::Error::build(false, &[], self.enviroment()))
        }
    }

    fn parse_DoubleFloatLiteral(
        &mut self,
    ) -> Result<generated_ast::DoubleFloatLiteral, Self::Error> {
        if let Some(Token::Literal(Literal::DoubleFloatLiteral(value))) = self.peek::<0>() {
            self.consume_token();
            Ok(generated_ast::DoubleFloatLiteral { value })
        } else {
            Err(Self::Error::build(false, &[], self.enviroment()))
        }
    }

    fn parse_DoubleIntLiteral(&mut self) -> Result<generated_ast::DoubleIntLiteral, Self::Error> {
        if let Some(Token::Literal(Literal::DoubleIntegerLiteral(value))) = self.peek::<0>() {
            self.consume_token();
            Ok(generated_ast::DoubleIntLiteral { value })
        } else {
            Err(Self::Error::build(false, &[], self.enviroment()))
        }
    }
}
