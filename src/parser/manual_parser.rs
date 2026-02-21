use crate::compiler::arena::Arena;
use crate::compiler::arena::ArenaIter;
use crate::parser::base_parser::BaseParser;
use crate::parser::errors::IParseErr;
use crate::parser::generated_parser::GeneratedParser;
use crate::parser::Parser;
use crate::tokenizer::tokens::Literal;
use crate::tokenizer::tokens::Token;

impl GeneratedParser for Parser<'_> {
    fn parse_Identifier(&mut self) -> Result<super::generated_ast::Identifier, Self::Error> {
        if let Some(Token::Identifier(symbol)) = self.peek::<0>() {
            Ok(super::generated_ast::Identifier { symbol })
        } else {
            Err(Self::Error::build(
                self.get_errors_arena(),
                true,
                [],
                self.enviroment(),
            ))
        }
    }
    fn parse_StringLiteral(&mut self) -> Result<super::generated_ast::StringLiteral, Self::Error> {
        if let Some(Token::Literal(Literal::StringLiteral(span))) = self.peek::<0>() {
            Ok(super::generated_ast::StringLiteral { span })
        } else {
            Err(Self::Error::build(
                self.get_errors_arena(),
                false,
                [],
                self.enviroment(),
            ))
        }
    }
    fn parse_DocComment(&mut self) -> Result<super::generated_ast::DocComment, Self::Error> {
        unimplemented!()
    }

    fn parse_CharLiteral(&mut self) -> Result<super::manual_ast::CharLiteral, Self::Error> {
        unimplemented!()
    }

    fn parse_IntLiteral(&mut self) -> Result<super::manual_ast::IntLiteral, Self::Error> {
        unimplemented!()
    }

    fn parse_FloatLiteral(&mut self) -> Result<super::manual_ast::FloatLiteral, Self::Error> {
        unimplemented!()
    }

    fn parse_BoolLiteral(&mut self) -> Result<super::manual_ast::BoolLiteral, Self::Error> {
        unimplemented!()
    }
}
