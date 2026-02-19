//! token types

use crate::compiler::span::Span;
use crate::compiler::symbol::Symbol;
use crate::utility::hashable_float::HashableFloat;

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
pub enum Token {
    Keyword(Keyword),
    Identifier(Symbol),
    Literal(Literal),
    Operator(Operator),
    Comment(Comment),
    Delimiter(Delimiter),
    EndOfFile,
}

/// Represents the keywords in the Hydent programming language.
#[derive(Debug, PartialEq, Clone, Copy, Hash)]
#[repr(u8)]
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

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
pub enum Literal {
    IntegerLiteral(i32),
    FloatLiteral(HashableFloat<f32>),
    DoubleIntegerLiteral(i64),
    DoubleFloatLiteral(HashableFloat<f64>),
    StringLiteral(Span),
    CharLiteral(char),
    BoolLiteral(bool),
}

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
#[repr(u8)]
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

/// Represents the delimiters in the Hydent programming language.
#[derive(Debug, PartialEq, Clone, Copy, Hash)]
#[repr(u8)]
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

/// Represents the different types of comments in the Hydent programming language.
#[derive(Debug, PartialEq, Clone, Copy, Hash)]
pub enum Comment {
    DocComment(Span), // `/// ...`
    LineComment,      // `// ...`
    BlockComment,     // `/* ... */`
}

impl Token {
    pub const fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Comment(_) => write!(f, "comment"),
            Self::Delimiter(delimiter) => write!(f, "{}", delimiter),
            Self::EndOfFile => write!(f, "EOF"),
            Self::Identifier(_) => write!(f, "identifier"),
            Self::Keyword(keyword) => write!(f, "{} keyword", keyword),
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::Operator(operator) => write!(f, "{} operator", operator),
        }
    }
}

impl std::fmt::Display for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Semicolon => write!(f, ";"),
            Self::LeftBrace => write!(f, "{{"),
            Self::RightBrace => write!(f, "}}"),
            Self::LeftParen => write!(f, "("),
            Self::RightParen => write!(f, ")"),
            Self::Comma => write!(f, ","),
            Self::LeftBracket => write!(f, "["),
            Self::RightBracket => write!(f, "]"),
        }
    }
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoubleFloat => write!(f, "DoubleFloat"),
            Self::DoubleInt => write!(f, "DoubleInt"),
            Self::Protocol => write!(f, "protocol"),
            Self::Continue => write!(f, "continue"),
            Self::Import => write!(f, "import"),
            Self::Static => write!(f, "static"),
            Self::Struct => write!(f, "struct"),
            Self::Extern => write!(f, "extern"),
            Self::Panics => write!(f, "panics"),
            Self::Module => write!(f, "module"),
            Self::Return => write!(f, "return"),
            Self::Ignore => write!(f, "ignore"),
            Self::Typeof => write!(f, "typeof"),
            Self::Class => write!(f, "class"),
            Self::Async => write!(f, "async"),
            Self::Match => write!(f, "match"),
            Self::While => write!(f, "while"),
            Self::Await => write!(f, "await"),
            Self::Break => write!(f, "break"),
            Self::Const => write!(f, "const"),
            Self::Final => write!(f, "final"),
            Self::Float => write!(f, "Float"),
            Self::Usize => write!(f, "Usize"),
            Self::Never => write!(f, "Never"),
            Self::From => write!(f, "from"),
            Self::Enum => write!(f, "enum"),
            Self::Type => write!(f, "type"),
            Self::Else => write!(f, "else"),
            Self::Loop => write!(f, "loop"),
            Self::Pipe => write!(f, "pipe"),
            Self::This => write!(f, "this"),
            Self::Impl => write!(f, "impl"),
            Self::Bool => write!(f, "Bool"),
            Self::Char => write!(f, "Char"),
            Self::Void => write!(f, "Void"),
            Self::For => write!(f, "for"),
            Self::Let => write!(f, "let"),
            Self::Try => write!(f, "try"),
            Self::Mut => write!(f, "mut"),
            Self::Pub => write!(f, "pub"),
            Self::Int => write!(f, "Int"),
            Self::Any => write!(f, "Any"),
            Self::As => write!(f, "as"),
            Self::Fn => write!(f, "fn"),
            Self::If => write!(f, "if"),
            Self::In => write!(f, "in"),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IntegerLiteral(i) => write!(f, "integer literal {}", i),
            Self::FloatLiteral(fl) => write!(f, "float literal {}", fl),
            Self::DoubleIntegerLiteral(di) => write!(f, "double integer literal {}", di),
            Self::DoubleFloatLiteral(dfl) => write!(f, "double float literal {}", dfl),
            Self::StringLiteral(_) => write!(f, "string literal"),
            Self::CharLiteral(c) => write!(f, "char literal '{}'", c),
            Self::BoolLiteral(b) => write!(f, "boolean literal {}", b),
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RangeInclusive => write!(f, "..="),
            Self::FatArrow => write!(f, "=>"),
            Self::Pipe => write!(f, "|>"),
            Self::Arrow => write!(f, "->"),
            Self::NamespaceResolver => write!(f, "::"),
            Self::LogicalOr => write!(f, "||"),
            Self::LogicalAnd => write!(f, "&&"),
            Self::Equality => write!(f, "=="),
            Self::Inequality => write!(f, "!="),
            Self::LessThanOrEqual => write!(f, "<="),
            Self::GreaterThanOrEqual => write!(f, ">="),
            Self::ShiftLeft => write!(f, "<<"),
            Self::ShiftRight => write!(f, ">>"),
            Self::PowerOf => write!(f, "**"),
            Self::RangeExclusive => write!(f, ".."),
            Self::AddAssign => write!(f, "+="),
            Self::SubtractAssign => write!(f, "-="),
            Self::MultiplyAssign => write!(f, "*="),
            Self::DivideAssign => write!(f, "/="),
            Self::Multiply => write!(f, "*"),
            Self::Assignment => write!(f, "="),
            Self::Colon => write!(f, ":"),
            Self::At => write!(f, "@"),
            Self::Or => write!(f, "|"),
            Self::Xor => write!(f, "^"),
            Self::And => write!(f, "&"),
            Self::LessThan => write!(f, "<"),
            Self::GreaterThan => write!(f, ">"),
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Divide => write!(f, "/"),
            Self::Remainder => write!(f, "%"),
            Self::Not => write!(f, "!"),
            Self::BitwiseNot => write!(f, "~"),
            Self::MemberAccess => write!(f, "."),
            Self::Wildcard => write!(f, "_"),
        }
    }
}
