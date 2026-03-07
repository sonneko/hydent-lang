use crate::compiler::arena::{Arena, ArenaIter};
use crate::parser::base_parser::Enviroment;
use crate::{diagnostic::Diagnostic, tokenizer::tokens::Token};

#[derive(Clone, Copy, Debug)]
pub struct ParseErr {
    expected: &'static [Token],
    found: Enviroment,
}

pub trait IParseErr {
    fn build(identifier: bool, expected: &'static [Token], found: Enviroment) -> Self;
    fn is_endoffile_error(&self) -> bool;
}

impl IParseErr for ParseErr {
    fn build(identifier: bool, expected: &'static [Token], found: Enviroment) -> Self {
        Self { expected, found }
    }

    fn is_endoffile_error(&self) -> bool {
        self.found.current == Token::EndOfFile
    }
}
