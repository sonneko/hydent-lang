use crate::compiler::arena::{Arena, ArenaIter};
use crate::parser::base_parser::Enviroment;
use crate::{diagnostic::CompilerDiagnostic, tokenizer::tokens::Token};

#[derive(Clone, Copy)]
pub struct ParseErr {
    expected: ArenaIter<Token>,
    found: Enviroment,
}

pub trait IParseErr {
    fn build<const N: usize>(
        arena: &Arena,
        identifier: bool,
        expected: [Token; N],
        found: Enviroment,
    ) -> Self;
}

impl IParseErr for ParseErr {
    fn build<const N: usize>(
        arena: &Arena,
        identifier: bool,
        expected: [Token; N],
        found: Enviroment,
    ) -> Self {
        Self {
            expected: arena.alloc_slice(expected),
            found,
        }
    }
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl From<ParseErr> for Box<dyn CompilerDiagnostic> {
    fn from(value: ParseErr) -> Self {
        unimplemented!()
    }
}
