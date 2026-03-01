use crate::{
    compiler::{span::Span, symbol::Symbol},
    parser::ast_node::{ASTNode, TokenBitMap},
    parser::errors::ParseErr,
};

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct Identifier {
    pub symbol: Symbol,
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct StringLiteral {
    pub span: Span,
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct CharLiteral {}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct IntLiteral {}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct FloatLiteral {}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct BoolLiteral {}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct DocComment {
    pub(super) span: Span,
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct LineComment {}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct BlockComment {}
