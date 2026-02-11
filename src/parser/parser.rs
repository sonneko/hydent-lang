use std::iter::Peekable;

use crate::compiler::arena::{Arena, ArenaBox, ArenaIter};
use crate::compiler::context::frontend::CompilerFrontendContext;
use crate::compiler::symbol::Symbol;
use crate::parser::errors::ParseErr;
use crate::parser::generated_ast::Module;
use crate::parser::generated_parser::GeneratedParser;
use crate::tokenizer::tokens::{Delimiter, Keyword, Literal, Operator, Token};
use crate::utility::peekable_n::PeekableN;

pub trait BaseParser {
    type Error;
    fn peek_n<const N: usize>(&self) -> Option<&Token>;
    fn consume_token(&mut self) -> Option<&Token>;
    fn expect_token(&mut self, expected: Token) -> Result<(), Self::Error>;
    fn alloc_iter<T: Copy>(
        &mut self,
        hook: impl FnOnce() -> Result<T, Self::Error>,
    ) -> ArenaIter<T>;
    fn alloc_box<T: Copy>(&mut self, item: T) -> ArenaBox<T>;
}

impl<I> BaseParser for Parser<'_, I>
where
    I: Iterator<Item = Token>,
{
    type Error = ParseErr;
    fn alloc_box<T: Copy>(&mut self, value: T) -> ArenaBox<T> {
        self.ctx.arena.alloc(value)
    }

    fn alloc_iter<T: Copy>(
        &mut self,
        hook: impl FnMut() -> Result<T, Self::Error>,
    ) -> ArenaIter<T> {
        self.ctx.arena.alloc_with(hook)
    }

    fn peek_token<const N: usize>(&self) -> Option<&Token> {
        self.tokens.peek_n()
    }
}

pub struct Parser<'ctx, I>
where
    I: Iterator<Item = Token>,
{
    ctx: CompilerFrontendContext<'ctx>,
    tokens: PeekableN<I, Token, 2>,
}

impl GeneratedParser for Parser {}

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
