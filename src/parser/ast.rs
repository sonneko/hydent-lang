use crate::compiler::arena::{ArenaBox, ArenaIter};
use crate::compiler::symbol::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ast {
    pub declarations: ArenaIter<TopLevelDeclaration>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLevelDeclaration {
    Function(ArenaBox<FunctionDeclaration>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionDeclaration {
    pub name: Symbol,
    pub params: ArenaIter<FunctionParameter>,
    pub return_type: Option<TypeLiteral>,
    pub body: ArenaBox<BlockExpression>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionParameter {
    pub name: Symbol,
    pub type_literal: TypeLiteral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockExpression {
    pub statements: ArenaIter<Statement>,
    pub final_expr: Option<ArenaBox<Expression>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Statement {
    Expression(ArenaBox<Expression>),
    Assignment(ArenaBox<AssignmentStatement>),
    Return(ArenaBox<ReturnStatement>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Expression {
    Literal(Literal),
    FunctionCall(ArenaBox<FunctionCall>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeLiteral {
    Int, // 将来的に Enum を拡張して他の型を追加
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssignmentStatement {
    pub name: Symbol,
    pub value: ArenaBox<Expression>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReturnStatement {
    pub value: Option<ArenaBox<Expression>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionCall {
    pub name: Symbol,
    pub args: ArenaIter<Expression>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Literal {
    Integer(i32),
}
