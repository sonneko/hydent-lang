use crate::tokenizer::Token;

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