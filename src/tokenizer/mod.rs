pub mod errors;
mod test;
pub mod tokenizer;

use crate::common::span::Span;
use crate::common::symbol::Symbol;

/// Token types
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Keyword(Keyword),
    Identifier(Symbol),
    Literal(Literal),
    Operator(Operator),
    Comment(Comment),
    Delimiter(Delimiter),
    EndOfFile,
}

/// Keywords
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    DoubleFloat,
    DoubleInt,
    Protocol,
    Continue,
    Import,
    Static,
    Struct,
    Extern,
    Panics,
    Module,
    Return,
    Ignore,
    Typeof,
    Class,
    Async,
    Match,
    While,
    Await,
    Break,
    Const,
    Final,
    Float,
    Usize,
    Never,
    From,
    Enum,
    Type,
    Else,
    Loop,
    Pipe,
    This,
    Impl,
    Bool,
    Char,
    Void,
    For,
    Let,
    Try,
    Mut,
    Pub,
    Int,
    Any,
    As,
    Fn,
    If,
    In,
}

/// Literals
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Literal {
    IntegerLiteral(i32),
    FloatLiteral(f32),
    DoubleIntegerLiteral(i64), // TODO: implement parsing DoubleInt and DoubleFloat literal logic in tokenizer.rs
    DoubleFloatLiteral(f64),
    StringLiteral(Span),
    CharLiteral(char),
    BoolLiteral(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    RangeInclusive,     // ..=
    FatArrow,           // =>
    Pipe,               // |>
    Arrow,              // ->
    NamespaceResolver,  // ::
    LogicalOr,          // ||
    LogicalAnd,         // &&
    Equality,           // ==
    Inequality,         // !=
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=
    ShiftLeft,          // <<
    ShiftRight,         // >>
    PowerOf,            // **
    RangeExclusive,     // ..
    AddAssign,          // +=
    SubtractAssign,     // -=
    MultiplyAssign,     // *=
    DivideAssign,       // /=    
    Multiply,           // *
    Assignment,         // =
    Colon,              // :
    At,                 // @
    Or,                 // |
    Xor,                // ^
    And,                // &
    LessThan,           // <
    GreaterThan,        // >
    Add,                // +
    Subtract,           // -
    Divide,             // /
    Remainder,          // %
    Not,                // !
    BitwiseNot,         // ~
    MemberAccess,       // .
    Wildcard,           // _
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Delimiter {
    Semicolon,    // ;
    LeftBrace,    // {
    RightBrace,   // }
    LeftParen,    // (
    RightParen,   // )
    Comma,        // ,
    LeftBracket,  // [
    RightBracket, // ]
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Comment {
    DocComment(Span), // `/// ...`
    LineComment,         // `// ...`
    BlockComment,        // `/* ... */`
}
