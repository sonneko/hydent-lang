use crate::compiler::arena::Arena;
use crate::compiler::arena::ArenaIter;
use crate::parser::generated_parser::GeneratedParser;
use crate::parser::Parser;
use crate::tokenizer::tokens::Token;

impl<I> GeneratedParser for Parser<'_, I>
where
    I: Iterator<Item = Token>,
{
    fn parse_Identifier(&mut self) -> Result<super::manual_ast::Identifier, Self::Error> {
        unimplemented!()
    }
    fn comma_separated_exprs(
        &mut self,
    ) -> Result<ArenaIter<super::generated_ast::Expression>, Self::Error> {
        unimplemented!()
    }
    fn comma_separated_params(
        &mut self,
    ) -> Result<ArenaIter<super::generated_ast::Parameter>, Self::Error> {
        unimplemented!()
    }
    fn parse_StringLiteral(&mut self) -> Result<super::manual_ast::StringLiteral, Self::Error> {
        unimplemented!()
    }
}
