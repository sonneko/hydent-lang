//! tokenize errors enum

use std::fmt::Debug;

use crate::diagnostic::CompilerDiagnostic;

#[derive(PartialEq)]
pub enum TokenizeErr {
    StringLiteralNotClosed(usize),
    CharLiteralNotClosed(usize),
    InvalidCharLiteral(usize),
    InvalidIntegerLiteral(usize),
    InvalidFloatLiteral(usize),
    UnknownToken(usize),
    BlockCommentNotClosed(usize),
}

impl Debug for TokenizeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizeErr::StringLiteralNotClosed(index) => {
                write!(f, "String literal not closed at index {}", index)
            }
            TokenizeErr::CharLiteralNotClosed(index) => {
                write!(f, "Char literal not closed at index {}", index)
            }
            TokenizeErr::InvalidIntegerLiteral(index) => {
                write!(f, "Invalid integer literal at index {}", index)
            }
            TokenizeErr::InvalidFloatLiteral(index) => {
                write!(f, "Invalid float literal at index {}", index)
            }
            TokenizeErr::UnknownToken(index) => {
                write!(f, "Unknown token at index {}", index)
            }
            TokenizeErr::BlockCommentNotClosed(index) => {
                write!(f, "Block comment not closed at index {}", index)
            }
            TokenizeErr::InvalidCharLiteral(index) => {
                write!(f, "Invalid char literal at index {}", index)
            }
        }
    }
}

impl From<TokenizeErr> for Box<dyn CompilerDiagnostic> {
    fn from(value: TokenizeErr) -> Self {
        unimplemented!()
    }
}
