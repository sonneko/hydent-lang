//! tokenize errors enum

use crate::compiler::span::Span;

#[derive(PartialEq, Debug)]
pub enum TokenizeErrKind {
    StringLiteralNotClosed,
    CharLiteralNotClosed,
    InvalidCharLiteral,
    InvalidIntegerLiteral,
    InvalidFloatLiteral,
    UnknownToken,
    BlockCommentNotClosed,
}

#[derive(Debug)]
pub struct TokenizeErr {
    kind: TokenizeErrKind,
    span: Span,
}

impl TokenizeErr {
    pub fn new(kind: TokenizeErrKind, span: Span) -> Self {
        Self { kind, span }
    }
}
