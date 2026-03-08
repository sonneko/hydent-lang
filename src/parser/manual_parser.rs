use crate::diagnostic::stream::DiagnosticStream;
use crate::parser::base_parser::BaseParser;
use crate::parser::errors::IParseErr;
use crate::parser::generated_ast;
use crate::parser::generated_parser::GeneratedParser;
use crate::parser::tracer::Tracer;
use crate::parser::Parser;
use crate::tokenizer::tokens::Comment;
use crate::tokenizer::tokens::Literal;
use crate::tokenizer::tokens::Token;

impl<S: DiagnosticStream, TR: Tracer> GeneratedParser for Parser<'_, '_, '_, S, TR> {
    type TraceGuard = TR::Guard;
    fn trace(name: &'static str) -> TR::Guard {
        TR::trace(name)
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
