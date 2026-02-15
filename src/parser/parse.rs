use crate::compiler::arena::{Arena, ArenaBox, ArenaIter};
use crate::compiler::context::frontend::CompilerFrontendContext;
use crate::parser::errors::{IParseErr, ParseErr};
use crate::parser::generated_ast::Module;
use crate::parser::generated_parser::GeneratedParser;
use crate::tokenizer::tokens::Token;
use crate::utility::peekable_n::PeekableN;

pub struct Parser<'ctx, I>
where
    I: Iterator<Item = Token>,
{
    pub ctx: CompilerFrontendContext<'ctx>,
    pub tokens: PeekableN<I, Token, 2>,
}

impl<I> Parser<'_, I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: I, ctx: CompilerFrontendContext<'_>) -> Self {
        unimplemented!()
    }

    pub fn parse(self) -> Result<Module, ParseErr> {
        unimplemented!()
    }
}
