use std::any::Any;

use crate::compiler::arena::{Arena, ArenaBox, ArenaIter};
use crate::compiler::span::Span;
use crate::parser::ast_node::ASTNode;
use crate::parser::errors::{IParseErr, ParseErr};
use crate::parser::parse::Parser;
use crate::tokenizer::tokens::Token;

pub trait BaseParser: Sized {
    type Error: IParseErr;

    fn peek<const N: usize>(&self) -> Option<Token>;

    fn consume_token(&mut self) -> Option<Token>;

    fn expect(&mut self, expected: &'static Token) -> Result<(), Self::Error>;

    fn repeat<T: ASTNode>(
        &mut self,
        parser_fn: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<ArenaIter<T>, Self::Error>;

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

    fn now_span(&self) -> Span;
}

#[derive(Clone, Copy, Debug)]
pub struct Enviroment {
    pub current: Token,
    pub span: Span,
}

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
    ) -> Result<ArenaIter<T>, ParseErr> {
        self.ctx.ast_arena.start_iter_allocation::<T>();
        loop {
            let next_token = self.peek::<0>();
            if next_token == Some(Token::EndOfFile) || next_token.is_none() {
                break Ok(self.ctx.ast_arena.finish_iter_allocation::<T>());
            }
            match (
                T::is_first_sets(&next_token),
                T::is_follow_sets(&next_token),
            ) {
                (true, true) => {
                    self.ctx.ast_arena.finish_iter_allocation::<T>();
                    // WARNING: you should cover with manual_parser.
                    panic!("Internal Error: Invalid grammar. {:?}", next_token)
                }
                (true, false) => {
                    // list continue
                    match parser_fn(self) {
                        Ok(node) => {
                            self.ctx.ast_arena.alloc_iter_item(&node);
                        }
                        Err(err) => {
                            self.report_error(err);
                            if let Some(placeholder) = T::get_error_situation(err) {
                                self.ctx.ast_arena.alloc_iter_item(&placeholder);
                            }
                            while let Some(t) = self.peek::<0>() {
                                if T::is_sync_point(&Some(t)) {
                                    break;
                                }
                                self.consume_token();
                            }
                        }
                    }
                }
                (false, true) => {
                    // end of list
                    let iter = self.ctx.ast_arena.finish_iter_allocation::<T>();
                    return Ok(iter);
                }
                (false, false) => {
                    // must occure error
                    let err = parser_fn(self).unwrap_err();
                    self.report_error(err);
                    self.consume_token();
                }
            }
        }
    }

    fn expect(&mut self, expected: &'static Token) -> Result<(), Self::Error> {
        let found = self.peek::<0>();
        if let Some(found) = found {
            if found == *expected {
                self.consume_token();
                Ok(())
            } else {
                Err(ParseErr::build(
                    self.get_errors_arena(),
                    matches!(expected, Token::Identifier(_)),
                    std::array::from_ref(expected),
                    self.enviroment(),
                ))
            }
        } else {
            Err(ParseErr::build(
                self.get_errors_arena(),
                matches!(expected, Token::Identifier(_)),
                std::array::from_ref(expected),
                self.enviroment(),
            ))
        }
    }

    fn report_error(&self, err: Self::Error) {
        // TODO: add error to error_pool
        println!("occured error: {:?}", err);
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
        Enviroment {
            current: self.peek::<0>().unwrap_or(Token::EndOfFile),
            span: self.now_span(),
        }
    }

    fn now_span(&self) -> Span {
        self.tokens.get_now_span()
    }
}
