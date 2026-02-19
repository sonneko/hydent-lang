use crate::compiler::arena::Arena;
use crate::compiler::arena::ArenaIter;
use crate::parser::generated_parser::GeneratedParser;
use crate::parser::Parser;
use crate::tokenizer::tokens::Token;

impl<I> GeneratedParser for Parser<'_, I>
where
    I: Iterator<Item = Token>,
{
    fn parse_Identifier(&mut self) -> Result<super::generated_ast::Identifier, Self::Error> {
        unimplemented!()
    }
    fn parse_StringLiteral(&mut self) -> Result<super::generated_ast::StringLiteral, Self::Error> {
        unimplemented!()
    }
}
