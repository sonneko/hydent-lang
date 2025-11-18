pub mod errors;
mod test;
pub mod tokenizer;

/// Token types
#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Keyword(Keyword),
    Identifier(&'a str),
    Literal(Literal<'a>),
    Operator(Operator),
    Comment(Comment<'a>),
    Delimiter(Delimiter),
    EndOfFile,
}

/// Keywords
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub enum Literal<'a> {
    IntegerLiteral(i32),
    FloatLiteral(f32),
    DoubleIntegerLiteral(i64), // TODO: implement parsing DoubleInt and DoubleFloat literal logic in tokenizer.rs
    DoubleFloatLiteral(f64),
    StringLiteral(&'a str),
    CharLiteral(char),
    BoolLiteral(bool),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Comment<'a> {
    DocComment(&'a str), // `/// ...`
    LineComment,         // `// ...`
    BlockComment,        // `/* ... */`
}
