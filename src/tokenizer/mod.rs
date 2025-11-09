pub mod tokenizer;
mod test;
pub mod errors;

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
    Import,
    From,
    Fn,
    Const,
    Let,
    Try,
    Class,
    Return,
    Pub,
    Static,
    Final,
    If,
    Else,
    For,
    In,
    While,
    Break,
    Continue,
    Match,
    Protocol,
}

/// Literals
#[derive(Debug, PartialEq)]
pub enum Literal<'a> {
    IntegerLiteral(i32),
    FloatLiteral(f32),
    DoubleIntegerLiteral(i64),
    DoubleFloatLiteral(f64),
    StringLiteral(&'a str),
    CharLiteral(char),
    BoolLiteral(bool),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Pow,      // **
    Divide,   // /
    Modulo,   // %

    AddAssign, // +=
    SubAssign, // -=
    MulAssign, // *=
    PowAssign, // **=
    DivAssign, // /=
    ModAssign, // %=

    Equal,        // ==
    NotEqual,     // !=
    Greater,      // >
    Less,         // <
    GreaterEqual, // >=
    LessEqual,    // <=

    And, // &&
    Or,  // ||
    Not, // !

    Increment, // ++
    Decrement, // --

    CommaComma, // ..

    Assign, // =

}

#[derive(Debug, PartialEq)]
pub enum Comment<'a> {
    DocComment(&'a str), // `/// ...`
    LineComment,        // `// ...`
    BlockComment,       // `/* ... */`
}

#[derive(Debug, PartialEq)]
pub enum Delimiter {
    Comma,      // ,
    Dot,        // .
    Colon,      // :
    ColonColon, // ::
    Semicolon,  // ;

    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    At,        // @
    Backslash, // \
}