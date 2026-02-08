use std::iter::Peekable;

use crate::compiler::arena::{Arena, ArenaBox, ArenaIter};
use crate::compiler::context::frontend::CompilerFrontendContext;
use crate::compiler::symbol::Symbol;
use crate::parser::errors::ParseErr;
use crate::parser::generated_ast::Module;
use crate::parser::generated_parser::GeneratedParser;
use crate::tokenizer::tokens::{Delimiter, Keyword, Literal, Operator, Token};

pub trait BaseParser {
    type Error;
    fn peek_token(&self) -> Option<&Token>;
    fn consume_token(&mut self) -> Option<&Token>;
    fn expect_token(&mut self, expected: Token) -> Result<(), Self::Error>;
    fn report_error(&mut self, msg: String) -> Self::Error;
}

pub struct Parser;

impl GeneratedParser for Parser {}

impl Parser {
    pub fn new<I: Iterator<Item = Token>>(
        tokens: Peekable<I>,
        ctx: CompilerFrontendContext<'_>,
    ) -> Self {
        Parser
    }

    pub fn parse(self) -> Result<Module, ParseErr> {
        unimplemented!()
    }
}
