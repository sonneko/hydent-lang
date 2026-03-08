use crate::{
    compiler::{span::Span, symbol::Symbol},
    utility::hashable_float::HashableFloat,
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
pub struct CharLiteral {
    pub value: char,
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct IntLiteral {
    pub value: i32,
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct DoubleIntLiteral {
    pub value: i64,
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct FloatLiteral {
    pub value: HashableFloat<f32>,
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct DoubleFloatLiteral {
    pub value: HashableFloat<f64>,
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct BoolLiteral {
    pub value: bool,
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct DocComment {
    pub(super) span: Span,
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct LineComment {}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct BlockComment {}
