//! This module defines the error types that can occur during the tokenization phase of the Aya compiler.
//! It includes the `TokenizeErr` enum, which enumerates various issues like unclosed literals
//! or invalid characters, and provides a `Debug` implementation for formatted error messages.

use std::fmt::Debug;

/// Represents the different types of errors that can occur during tokenization.
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


/// Implements the `Debug` trait for `TokenizeErr` to provide user-friendly error messages.
impl Debug for TokenizeErr {
    /// Implements the `Debug` trait for `TokenizeErr` to provide user-friendly error messages.
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
