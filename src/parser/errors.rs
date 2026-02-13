use crate::compiler::arena::{Arena, ArenaIter};
use crate::{diagnostic::CompilerDiagnostic, tokenizer::tokens::Token};

#[derive(Clone, Copy)]
pub struct ParseErr {
    expected: ArenaIter<Token>,
    found: Option<Token>,
}

pub trait IParseErr {
    fn create<const N: usize>(arena: &Arena, expected: [Token; N], found: Option<&Token>) -> Self;
}

impl IParseErr for ParseErr {
    fn create<const N: usize>(arena: &Arena, expected: [Token; N], found: Option<&Token>) -> Self {
        Self {
            expected: arena.alloc_slice(expected),
            found: found.copied(),
        }
    }
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Into<Box<dyn CompilerDiagnostic>> for ParseErr {
    fn into(self) -> Box<dyn CompilerDiagnostic> {
        unimplemented!()
    }
}
