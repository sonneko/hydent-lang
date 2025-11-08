pub mod tokenizer;

/// Token types
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Literal(Literal),
    Operator(Operator),
    Comment(Comment),
}

/// Keywords
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
    For,
    In,
    While,
    Break,
    Continue,
    Match,
    Protocol,
}

/// Literals
pub enum Literal {
    IntegerLiteral(i32),
    FloatLiteral(f32),
    DoubleIntegerLiteral(i64),
    DoubleFloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),
    BoolLiteral(bool),
}

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

    Assign, // =

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

pub enum Comment {
    DocComment(String), // `/// ...`
    LineComment,        // `// ...`
    BlockComment,       // `/* ... */`
}
