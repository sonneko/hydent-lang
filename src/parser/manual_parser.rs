use crate::compiler::arena::Arena;
use crate::compiler::arena::ArenaIter;
use crate::parser::base_parser::BaseParser;
use crate::parser::errors::IParseErr;
use crate::parser::generated_parser::GeneratedParser;
use crate::parser::Parser;
use crate::tokenizer::tokens::Literal;
use crate::tokenizer::tokens::Token;

impl<I> GeneratedParser for Parser<'_, I>
where
    I: Iterator<Item = Token>,
{
    fn parse_Identifier(&mut self) -> Result<super::generated_ast::Identifier, Self::Error> {
        if let Some(Token::Identifier(symbol)) = self.peek::<0>() {
            Ok(super::generated_ast::Identifier { symbol: *symbol })
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
            Ok(super::generated_ast::StringLiteral { span: *span })
        } else {
            Err(Self::Error::build(
                self.get_errors_arena(),
                false,
                [],
                self.enviroment(),
            ))
        }
    }
}
