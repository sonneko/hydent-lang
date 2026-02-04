use crate::{diagnostic::CompilerDiagnostic, tokenizer::tokens::Token};

pub enum ParseErr {
    UnexpectedEndOfFile,
    UnexpectedToken(Token),
    ExpectIdentifier,
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