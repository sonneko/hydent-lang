use crate::compiler::arena::{Arena, ArenaBox, ArenaIter};
use crate::parser::ast_node::ASTNode;
use crate::parser::errors::{IParseErr, ParseErr};
use crate::parser::parse::Parser;
use crate::tokenizer::tokens::Token;

pub trait BaseParser: Sized {
    type Error: IParseErr;
    fn peek<const N: usize>(&self) -> Option<Token>;
    fn consume_token(&mut self) -> Option<Token>;
    fn expect(&mut self, expected: Token) -> Result<(), Self::Error>;
    fn repeat<T: ASTNode>(
        &mut self,
        parser_fn: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> ArenaIter<T>;
    fn alloc_box<T: ASTNode>(
        &mut self,
        parser_fn: impl FnOnce(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<ArenaBox<T>, Self::Error>;
    fn get_errors_arena(&self) -> &Arena;
    fn report_error(&self, error: Self::Error);
    fn backtrack<T: ASTNode>(
        &mut self,
        parser_fn: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<T, Self::Error>;
    fn enviroment(&self) -> Enviroment;
}

#[derive(Clone, Copy)]
pub struct Enviroment {}

impl BaseParser for Parser<'_> {
    type Error = ParseErr;
    fn alloc_box<T: ASTNode>(
        &mut self,
        parser_fn: impl FnOnce(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<ArenaBox<T>, Self::Error> {
        Ok(self.ctx.ast_arena.alloc(parser_fn(self)?))
    }

    fn peek<const N: usize>(&self) -> Option<Token> {
        self.tokens.peek(N).map(|(token, _)| token)
    }

    fn consume_token(&mut self) -> Option<Token> {
        self.tokens.next().map(|(token, _)| token)
    }

    fn get_errors_arena(&self) -> &Arena {
        self.ctx.errors_arena
    }

    fn repeat<T: ASTNode>(
        &mut self,
        mut parser_fn: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> ArenaIter<T> {
        self.ctx.ast_arena.alloc_with(|| match parser_fn(self) {
            Ok(value) => Some(value),
            Err(err) => {
                self.report_error(err);
                let ret = Some(T::get_error_situation(err)?);
                while !T::is_sync_point(self.peek::<0>().as_ref()) {
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
        // TODO: add error to error_pool
        unimplemented!()
    }

    fn backtrack<T: ASTNode>(
        &mut self,
        mut parser_fn: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<T, Self::Error> {
        self.tokens.checkpoint();
        let node = parser_fn(self);
        match node {
            Ok(_) => {
                self.tokens.commit();
            }
            Err(_) => {
                self.tokens.rollback();
            }
        }
        node
    }

    fn enviroment(&self) -> Enviroment {
        // TODO: consider Environment structure
        unimplemented!()
    }
}
