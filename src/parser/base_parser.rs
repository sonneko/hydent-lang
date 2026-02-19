use crate::compiler::arena::{Arena, ArenaBox, ArenaIter};
use crate::parser::ast::ASTNode;
use crate::parser::errors::{IParseErr, ParseErr};
use crate::parser::parse::Parser;
use crate::tokenizer::tokens::Token;

pub trait BaseParser {
    type Error: IParseErr;
    fn peek<const N: usize>(&self) -> Option<&Token>;
    fn consume_token(&mut self) -> Option<Token>;
    fn expect(&mut self, expected: Token) -> Result<(), Self::Error>;
    fn repeat<T: ASTNode>(
        &mut self,
        hook: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> ArenaIter<T>;
    fn alloc_box<T: ASTNode>(&mut self, item: T) -> ArenaBox<T>;
    fn get_errors_arena(&self) -> &Arena;
    fn report_error(&self, error: Self::Error);
    fn backtrack<T: ASTNode>(
        &mut self,
        hook: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<T, Self::Error>;
    fn enviroment(&self) -> Enviroment;
}

#[derive(Clone, Copy)]
pub struct Enviroment {}

impl<I> BaseParser for Parser<'_, I>
where
    I: Iterator<Item = Token>,
{
    type Error = ParseErr;
    fn alloc_box<T: ASTNode>(&mut self, value: T) -> ArenaBox<T> {
        self.ctx.ast_arena.alloc(value)
    }

    fn peek<const N: usize>(&self) -> Option<&Token> {
        self.tokens.peek_n::<N>()
    }

    fn consume_token(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn get_errors_arena(&self) -> &Arena {
        self.ctx.errors_arena
    }

    fn repeat<T: ASTNode>(
        &mut self,
        mut hook: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> ArenaIter<T> {
        self.ctx.ast_arena.alloc_with(|| match hook(self) {
            Ok(value) => Some(value),
            Err(err) => {
                self.report_error(err);
                let ret = Some(T::get_error_situation(err)?);
                while !T::is_sync_point(self.peek::<1>()) {
                    self.consume_token();
                }
                self.consume_token();
                ret
            }
        })
    }

    fn expect(&mut self, expected: Token) -> Result<(), Self::Error> {
        let found = self.consume_token();
        if let Some(found) = found {
            if found == expected {
                Ok(())
            } else {
                Err(ParseErr::build(
                    self.get_errors_arena(),
                    matches!(expected, Token::Identifier(_)),
                    [expected],
                    self.enviroment(),
                ))
            }
        } else {
            Err(ParseErr::build(
                self.get_errors_arena(),
                matches!(expected, Token::Identifier(_)),
                [expected],
                self.enviroment(),
            ))
        }
    }

    fn report_error(&self, err: Self::Error) {
        // TODO: implement
        unimplemented!()
    }

    fn backtrack<T: ASTNode>(
        &mut self,
        hook: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<T, Self::Error> {
        // TODO: implement with buffer
        unimplemented!()
    }

    fn enviroment(&self) -> Enviroment {
        unimplemented!()
    }
}
