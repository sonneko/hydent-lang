use crate::compiler::arena::{Arena, ArenaIter};
use crate::parser::base_parser::Enviroment;
use crate::{diagnostic::CompilerDiagnostic, tokenizer::tokens::Token};

#[derive(Clone, Copy, Debug)]
pub struct ParseErr {
    expected: &'static [Token],
    found: Enviroment,
}

pub trait IParseErr {
    fn build<const N: usize>(
        arena: &Arena,
        identifier: bool,
        expected: &'static [Token; N],
        found: Enviroment,
    ) -> Self;
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
    }
}

impl From<ParseErr> for Box<dyn CompilerDiagnostic> {
    fn from(value: ParseErr) -> Self {
        // TODO: convert ParseErr into CompilerDiagnostic
        unimplemented!()
    }
}
