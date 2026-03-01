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
        Ok(loop {
            let parsed = parser_fn(self);
            if let Err(err) = parsed {
                if err.is_endoffile_error() {
                    self.ctx.ast_arena.finish_iter_allocation::<T>();
                    return Err(err);
                }

                // check if error occured because repeat item finish or because there is a syntax error

                // TODO: can't implement because of lack of information above.
                // Parser generator should inform the potential next tokens.

                // Now it never recover error for avoiding infinitely loop.

                // if [error occured because repeat item finish] {
                //     // need backtrace
                //     break self.ctx.ast_arena.finish_iter_allocation::<T>();
                // } else {
                //     // Let's recover error
                //     while !T::is_sync_point(self.peek::<0>().as_ref()) {
                //         self.consume_token();
                //     }
                //
                //     if let Some(placeholder) = T::get_error_situation(err) {
                //         self.ctx.ast_arena.alloc_iter_item(&placeholder);
                //     } else {
                //         // can't recover because T is struct. this error will recover above 1 layer.
                //         self.ctx.ast_arena.finish_iter_allocation::<T>();
                //         return Err(err);
                //     }
                // }

                break self.ctx.ast_arena.finish_iter_allocation::<T>();
            } else {
                self.ctx.ast_arena.alloc_iter_item(&parsed.unwrap());
            }
        })
    }

    fn expect(&mut self, expected: &'static Token) -> Result<(), Self::Error> {
        let found = self.consume_token();
        if let Some(found) = found {
            if found == *expected {
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
