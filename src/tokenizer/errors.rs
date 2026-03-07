//! tokenize errors enum

use crate::compiler::span::Span;

#[derive(PartialEq)]
pub enum TokenizeErrKind {
    StringLiteralNotClosed,
    CharLiteralNotClosed,
    InvalidCharLiteral,
    InvalidIntegerLiteral,
    InvalidFloatLiteral,
    UnknownToken,
    BlockCommentNotClosed,
}

pub struct TokenizeErr {
    kind: TokenizeErrKind,
    span: Span,
}

impl TokenizeErr {
    pub fn new(kind: TokenizeErrKind, span: Span) -> Self {
        Self { kind, span }
    }
}
