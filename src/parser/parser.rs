use crate::compiler::arena::{Arena, ArenaBox, ArenaIter};
use crate::compiler::context::frontend::CompilerFrontendContext;
use crate::parser::errors::{IParseErr, ParseErr};
use crate::parser::generated_ast::Module;
use crate::parser::generated_parser::GeneratedParser;
use crate::tokenizer::tokens::Token;
use crate::utility::peekable_n::PeekableN;

pub trait BaseParser {
    type Error: IParseErr;
    fn peek_n<const N: usize>(&self) -> Option<&Token>;
    fn consume_token(&mut self) -> Option<Token>;
    fn expect_token(&mut self, expected: Token) -> Result<(), Self::Error>;
    fn repeat<T: Copy>(
        &mut self,
        hook: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> ArenaIter<T>;
    fn alloc_box<T: Copy>(&mut self, item: T) -> ArenaBox<T>;
    fn get_errors_arena(&self) -> &Arena;
}

impl<I> BaseParser for Parser<'_, I>
where
    I: Iterator<Item = Token>,
{
    type Error = ParseErr;
    fn alloc_box<T: Copy>(&mut self, value: T) -> ArenaBox<T> {
        self.ctx.ast_arena.alloc(value)
    }

    fn peek_n<const N: usize>(&self) -> Option<&Token> {
        self.tokens.peek_n::<N>()
    }

    fn consume_token(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn get_errors_arena(&self) -> &Arena {
        self.ctx.errors_arena
    }

    fn repeat<T: Copy>(
        &mut self,
        mut hook: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> ArenaIter<T> {
        self.ctx.ast_arena.alloc_with(|| hook(self).ok())
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), Self::Error> {
        let found = self.consume_token();
        if let Some(found) = found {
            if found == expected {
                Ok(())
            } else {
                Err(ParseErr::create(
                    self.get_errors_arena(),
                    [expected],
                    Some(&found),
                ))
            }
        } else {
            Err(ParseErr::create(self.get_errors_arena(), [expected], None))
        }
    }
}

pub struct Parser<'ctx, I>
where
    I: Iterator<Item = Token>,
{
    ctx: CompilerFrontendContext<'ctx>,
    tokens: PeekableN<I, Token, 2>,
}

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
