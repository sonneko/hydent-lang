use crate::{
    compiler::{span::Span, symbol::Symbol},
    parser::ast_node::{ASTNode, SyncPointBitMap},
    parser::errors::ParseErr,
};

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct Identifier {
    pub symbol: Symbol,
}
impl ASTNode for Identifier {
    const SYNC_POINT_SETS: SyncPointBitMap = SyncPointBitMap::build_map(false, &[]);
    fn get_error_situation(err: ParseErr) -> Option<Self> {
        None
    }
    fn accept<V: super::ast::ASTVisitor>(&self, visitor: &mut V) {}
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct StringLiteral {
    pub span: Span,
}
impl ASTNode for StringLiteral {
    const SYNC_POINT_SETS: SyncPointBitMap = SyncPointBitMap::build_map(false, &[]);
    fn get_error_situation(err: ParseErr) -> Option<Self> {
        None
    }
    fn accept<V: super::ast::ASTVisitor>(&self, visitor: &mut V) {}
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct CharLiteral {}
impl ASTNode for CharLiteral {
    const SYNC_POINT_SETS: SyncPointBitMap = SyncPointBitMap::build_map(false, &[]);
    fn get_error_situation(err: ParseErr) -> Option<Self> {
        None
    }
    fn accept<V: super::ast::ASTVisitor>(&self, visitor: &mut V) {}
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct IntLiteral {}
impl ASTNode for IntLiteral {
    const SYNC_POINT_SETS: SyncPointBitMap = SyncPointBitMap::build_map(false, &[]);
    fn get_error_situation(err: ParseErr) -> Option<Self> {
        None
    }
    fn accept<V: super::ast::ASTVisitor>(&self, visitor: &mut V) {}
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct FloatLiteral {}
impl ASTNode for FloatLiteral {
    const SYNC_POINT_SETS: SyncPointBitMap = SyncPointBitMap::build_map(false, &[]);
    fn get_error_situation(err: ParseErr) -> Option<Self> {
        None
    }
    fn accept<V: super::ast::ASTVisitor>(&self, visitor: &mut V) {}
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct BoolLiteral {}
impl ASTNode for BoolLiteral {
    const SYNC_POINT_SETS: SyncPointBitMap = SyncPointBitMap::build_map(false, &[]);
    fn get_error_situation(err: ParseErr) -> Option<Self> {
        None
    }
    fn accept<V: super::ast::ASTVisitor>(&self, visitor: &mut V) {}
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct DocComment {
    pub(super) span: Span,
}
impl ASTNode for DocComment {
    const SYNC_POINT_SETS: SyncPointBitMap = SyncPointBitMap::build_map(false, &[]);
    fn get_error_situation(err: ParseErr) -> Option<Self> {
        None
    }
    fn accept<V: super::ast::ASTVisitor>(&self, visitor: &mut V) {}
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct LineComment {}
impl ASTNode for LineComment {
    const SYNC_POINT_SETS: SyncPointBitMap = SyncPointBitMap::build_map(false, &[]);
    fn get_error_situation(err: ParseErr) -> Option<Self> {
        None
    }
    fn accept<V: super::ast::ASTVisitor>(&self, visitor: &mut V) {}
}

#[derive(Debug, Copy, Clone, std::hash::Hash, PartialEq, Eq)]
pub struct BlockComment {}
impl ASTNode for BlockComment {
    const SYNC_POINT_SETS: SyncPointBitMap = SyncPointBitMap::build_map(false, &[]);
    fn get_error_situation(err: ParseErr) -> Option<Self> {
        None
    }
    fn accept<V: super::ast::ASTVisitor>(&self, visitor: &mut V) {}
}
