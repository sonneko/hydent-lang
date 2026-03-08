use crate::compiler::arena::{ArenaBox, ArenaIter};
use crate::compiler::span::Span;
use crate::diagnostic::stream::DiagnosticStream;
use crate::parser::ast_node::ASTNode;
use crate::parser::errors::{IParseErr, ParseErr};
use crate::parser::parse::Parser;
use crate::parser::recovery::recover;
use crate::parser::tracer::Tracer;
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

    fn alloc<T: ASTNode>(&mut self, node: T) -> ArenaBox<T>;

    fn report_error(&mut self, error: Self::Error);

    fn backtrack<T: ASTNode>(
        &mut self,
        parser_fn: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<T, Self::Error>;

    fn enviroment(&self) -> Enviroment;

    fn now_span(&self) -> Span;

    fn optional<T: ASTNode>(
        &mut self,
        parser_fn: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> Option<T>;

    fn optional_box<T: ASTNode<Target = T>>(
        &mut self,
        parser_fn: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> Option<ArenaBox<T>>;

    fn is_panic_or_backtrack_mode(&mut self) -> bool;

    fn set_panic_or_backtrack_mode(&mut self, mode: bool);
}

#[derive(Clone, Copy, Debug)]
pub struct Enviroment {
    pub current: Token,
    pub span: Span,
}

impl<S: DiagnosticStream, TR: Tracer> BaseParser for Parser<'_, '_, '_, S, TR> {
    type Error = ParseErr;
    fn alloc_box<T: ASTNode>(
        &mut self,
        parser_fn: impl FnOnce(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<ArenaBox<T>, Self::Error> {
        Ok(self.ctx.ast_arena.alloc(parser_fn(self)?))
    }

    fn alloc<T: ASTNode>(&mut self, node: T) -> ArenaBox<T> {
        self.ctx.ast_arena.alloc(node)
    }

    fn peek<const N: usize>(&self) -> Option<Token> {
        self.tokens.peek(N).map(|(token, _)| token)
    }

    fn consume_token(&mut self) -> Option<Token> {
        self.tokens.next().map(|(token, _)| token)
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
                T::is_first1_sets(&next_token),
                T::is_follow_sets(&next_token),
            ) {
                (true, true) => {
                    if !T::is_first2_sets(&self.peek::<1>()) {
                        break Ok(self.ctx.ast_arena.finish_iter_allocation::<T>());
                    }
                    match self.backtrack(&mut parser_fn) {
                        Ok(node) => {
                            self.ctx.ast_arena.alloc_iter_item(&node);
                        }
                        Err(err) => {
                            break Ok(self.ctx.ast_arena.finish_iter_allocation::<T>());
                        }
                    }
                }
                (true, false) => match parser_fn(self) {
                    Ok(node) => {
                        self.ctx.ast_arena.alloc_iter_item(&node);
                    }
                    Err(err) => {
                        self.report_error(err);
                        recover::<T, S, TR>(self);
                    }
                },
                (false, true) => {
                    break Ok(self.ctx.ast_arena.finish_iter_allocation::<T>());
                }
                (false, false) => {
                    //TODO:  build error and report
                    recover::<T, S, TR>(self);
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
                    matches!(expected, Token::Identifier(_)),
                    std::array::from_ref(expected),
                    self.enviroment(),
                ))
            }
        } else {
            Err(ParseErr::build(
                matches!(expected, Token::Identifier(_)),
                std::array::from_ref(expected),
                self.enviroment(),
            ))
        }
    }

    fn report_error(&mut self, err: Self::Error) {
        if !self.is_panic_or_backtrack_mode() {
            self.diagnostic_stream.pour(err, &self.enviroment());
        }
    }

    fn backtrack<T: ASTNode>(
        &mut self,
        mut parser_fn: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<T, Self::Error> {
        self.is_panic_or_backtrack_mode = true;
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
        self.is_panic_or_backtrack_mode = false;
        node
    }

    fn optional<T: ASTNode>(
        &mut self,
        mut parser_fn: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> Option<T> {
        if T::is_first1_sets(&self.peek::<0>()) {
            self.backtrack(parser_fn).ok()
        } else {
            None
        }
    }

    fn optional_box<T: ASTNode<Target = T>>(
        &mut self,
        mut parser_fn: impl FnMut(&mut Self) -> Result<T, Self::Error>,
    ) -> Option<ArenaBox<T>> {
        if T::is_first1_sets(&self.peek::<0>()) {
            self.backtrack(|this| this.alloc_box(|this| parser_fn(this)))
                .ok()
        } else {
            None
        }
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

    fn is_panic_or_backtrack_mode(&mut self) -> bool {
        self.is_panic_or_backtrack_mode
    }

    fn set_panic_or_backtrack_mode(&mut self, mode: bool) {
        self.is_panic_or_backtrack_mode = mode;
    }
}
