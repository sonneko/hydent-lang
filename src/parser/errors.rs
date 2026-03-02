use crate::compiler::arena::{Arena, ArenaIter};
use crate::parser::base_parser::Enviroment;
use crate::{diagnostic::CompilerDiagnostic, tokenizer::tokens::Token};

#[derive(Clone, Copy, Debug)]
pub struct ParseErr {
    expected: &'static [Token],
    found: Enviroment,
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected {:?}, found {:?}", self.expected, self.found.current)
    }
}

pub trait IParseErr {
    fn build<const N: usize>(
        arena: &Arena,
        identifier: bool,
        expected: &'static [Token; N],
        found: Enviroment,
    ) -> Self;
    fn is_endoffile_error(&self) -> bool;
}

impl IParseErr for ParseErr {
    fn build<const N: usize>(
        arena: &Arena,
        identifier: bool,
        expected: &'static [Token; N],
        found: Enviroment,
    ) -> Self {
        Self { expected, found }
    }

    fn is_endoffile_error(&self) -> bool {
        self.found.current == Token::EndOfFile
    }
}

impl From<ParseErr> for Box<dyn CompilerDiagnostic> {
    fn from(value: ParseErr) -> Self {
        // TODO: convert ParseErr into CompilerDiagnostic
        unimplemented!()
    }
}
