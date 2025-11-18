// tokenizer.rs (continued)

#[cfg(test)]
mod tests {
    use crate::tokenizer::{Token, Delimiter, Operator, Literal, Keyword, Comment};
    use crate::Tokenizer;

    fn assert_tokens_match(input: &str, expected_tokens: Vec<Token>) {
        let actual_tokens = Tokenizer::new(input).tokenize().unwrap();

        assert_eq!(actual_tokens, expected_tokens, "Failed for input: {}", input);
    }

    // --- テストケース ---

    #[test]
    fn test_empty_input() {
        assert_tokens_match("", vec![Token::EndOfFile]);
    }

    #[test]
    fn test_whitespace_only() {
        assert_tokens_match("   \t\n\r", vec![Token::EndOfFile]);
    }

    #[test]
    fn test_keywords() {
        let input = "DoubleFloat DoubleInt protocol continue import static struct extern panics module return ignore typeof class async match while await break const final Float Usize Never from enum type else loop pipe this impl Bool Char Void for let try mut pub Int Any as fn if in";
        let expected = vec![
            Token::Keyword(Keyword::DoubleFloat), Token::Keyword(Keyword::DoubleInt),
            Token::Keyword(Keyword::Protocol), Token::Keyword(Keyword::Continue),
            Token::Keyword(Keyword::Import), Token::Keyword(Keyword::Static),
            Token::Keyword(Keyword::Struct), Token::Keyword(Keyword::Extern),
            Token::Keyword(Keyword::Panics), Token::Keyword(Keyword::Module),
            Token::Keyword(Keyword::Return), Token::Keyword(Keyword::Ignore),
            Token::Keyword(Keyword::Typeof), Token::Keyword(Keyword::Class),
            Token::Keyword(Keyword::Async), Token::Keyword(Keyword::Match),
            Token::Keyword(Keyword::While), Token::Keyword(Keyword::Await),
            Token::Keyword(Keyword::Break), Token::Keyword(Keyword::Const),
            Token::Keyword(Keyword::Final), Token::Keyword(Keyword::Float),
            Token::Keyword(Keyword::Usize), Token::Keyword(Keyword::Never),
            Token::Keyword(Keyword::From), Token::Keyword(Keyword::Enum),
            Token::Keyword(Keyword::Type), Token::Keyword(Keyword::Else),
            Token::Keyword(Keyword::Loop), Token::Keyword(Keyword::Pipe),
            Token::Keyword(Keyword::This), Token::Keyword(Keyword::Impl),
            Token::Keyword(Keyword::Bool), Token::Keyword(Keyword::Char),
            Token::Keyword(Keyword::Void), Token::Keyword(Keyword::For),
            Token::Keyword(Keyword::Let), Token::Keyword(Keyword::Try),
            Token::Keyword(Keyword::Mut), Token::Keyword(Keyword::Pub),
            Token::Keyword(Keyword::Int), Token::Keyword(Keyword::Any),
            Token::Keyword(Keyword::As), Token::Keyword(Keyword::Fn),
            Token::Keyword(Keyword::If), Token::Keyword(Keyword::In),
            Token::EndOfFile,
        ];
        assert_tokens_match(input, expected);
    }

    #[test]
    fn test_identifiers() {
        let input = "my_var someIdentifier leading_underscore_123 keyword_suffix";
        let expected = vec![
            Token::Identifier("my_var"),
            Token::Identifier("someIdentifier"),
            Token::Identifier("leading_underscore_123"),
            Token::Identifier("keyword_suffix"),
            Token::EndOfFile,
        ];
        assert_tokens_match(input, expected);
    }

    #[test]
    fn test_literals_integers() {
        let input = "0 123 456789";
        let expected = vec![
            Token::Literal(Literal::IntegerLiteral(0)),
            Token::Literal(Literal::IntegerLiteral(123)),
            Token::Literal(Literal::IntegerLiteral(456789)),
            Token::EndOfFile,
        ];
        assert_tokens_match(input, expected);
    }

    #[test]
    fn test_literals_floats() {
        let input = "0.0 123.456 1000.001";
        let expected = vec![
            Token::Literal(Literal::FloatLiteral(0.0)),
            Token::Literal(Literal::FloatLiteral(123.456)),
            Token::Literal(Literal::FloatLiteral(1000.001)),
            Token::EndOfFile,
        ];
        assert_tokens_match(input, expected);
    }

    #[test]
    fn test_literals_strings() {
        let input = r#""hello" "world!" "" "こんにちは世界""#; // Raw string to avoid double escaping
        let expected = vec![
            Token::Literal(Literal::StringLiteral("hello")),
            Token::Literal(Literal::StringLiteral("world!")),
            Token::Literal(Literal::StringLiteral("")),
            Token::Literal(Literal::StringLiteral("こんにちは世界")), // マルチバイト文字列
            Token::EndOfFile,
        ];
        assert_tokens_match(input, expected);
    }

    #[test]
    fn test_literals_chars() {
        let input = "'a' 'Z' '1' 'あ'";
        let expected = vec![
            Token::Literal(Literal::CharLiteral('a')),
            Token::Literal(Literal::CharLiteral('Z')),
            Token::Literal(Literal::CharLiteral('1')),
            Token::Literal(Literal::CharLiteral('あ')), // マルチバイト文字
            Token::EndOfFile,
        ];
        assert_tokens_match(input, expected);
    }

    #[test]
    fn test_literals_booleans() {
        let input = "true false";
        let expected = vec![
            Token::Literal(Literal::BoolLiteral(true)),
            Token::Literal(Literal::BoolLiteral(false)),
            Token::EndOfFile,
        ];
        assert_tokens_match(input, expected);
    }

    #[test]
    fn test_operators() {
        let input = "..= => |> -> :: || && == != <= >= << >> ** .. += -= *= /= * = : @ | ^ & < > + - / % ! ~ . _";
        let expected = vec![
            Token::Operator(Operator::RangeInclusive),
            Token::Operator(Operator::FatArrow),
            Token::Operator(Operator::Pipe),
            Token::Operator(Operator::Arrow),
            Token::Operator(Operator::NamespaceResolver),
            Token::Operator(Operator::LogicalOr),
            Token::Operator(Operator::LogicalAnd),
            Token::Operator(Operator::Equality),
            Token::Operator(Operator::Inequality),
            Token::Operator(Operator::LessThanOrEqual),
            Token::Operator(Operator::GreaterThanOrEqual),
            Token::Operator(Operator::ShiftLeft),
            Token::Operator(Operator::ShiftRight),
            Token::Operator(Operator::PowerOf),
            Token::Operator(Operator::RangeExclusive),
            Token::Operator(Operator::AddAssign),
            Token::Operator(Operator::SubtractAssign),
            Token::Operator(Operator::MultiplyAssign),
            Token::Operator(Operator::DivideAssign),
            Token::Operator(Operator::Multiply),
            Token::Operator(Operator::Assignment),
            Token::Operator(Operator::Colon),
            Token::Operator(Operator::At),
            Token::Operator(Operator::Or),
            Token::Operator(Operator::Xor),
            Token::Operator(Operator::And),
            Token::Operator(Operator::LessThan),
            Token::Operator(Operator::GreaterThan),
            Token::Operator(Operator::Add),
            Token::Operator(Operator::Subtract),
            Token::Operator(Operator::Divide),
            Token::Operator(Operator::Remainder),
            Token::Operator(Operator::Not),
            Token::Operator(Operator::BitwiseNot),
            Token::Operator(Operator::MemberAccess),
            Token::Operator(Operator::Wildcard),
            Token::EndOfFile,
        ];
        assert_tokens_match(input, expected);
    }

    #[test]
    fn test_delimiters() {
        let input = "; { } ( ) , [ ]";
        let expected = vec![
            Token::Delimiter(Delimiter::Semicolon),
            Token::Delimiter(Delimiter::LeftBrace),
            Token::Delimiter(Delimiter::RightBrace),
            Token::Delimiter(Delimiter::LeftParen),
            Token::Delimiter(Delimiter::RightParen),
            Token::Delimiter(Delimiter::Comma),
            Token::Delimiter(Delimiter::LeftBracket),
            Token::Delimiter(Delimiter::RightBracket),
            Token::EndOfFile,
        ];
        assert_tokens_match(input, expected);
    }

    #[test]
    fn test_comments() {
        let input = r#"
/// Doc comment line 1
/// Doc comment line 2
// Line comment
/* Block comment */
/*
 * Multi-line
 * block comment
 */
let x = 1; // Inline comment
/* block comment */
"#;
        let expected = vec![
            Token::Comment(Comment::DocComment(" Doc comment line 1")),
            Token::Comment(Comment::DocComment(" Doc comment line 2")),
            Token::Comment(Comment::LineComment),
            Token::Comment(Comment::BlockComment),
            Token::Comment(Comment::BlockComment),
            Token::Keyword(Keyword::Let),
            Token::Identifier("x"),
            Token::Operator(Operator::Assignment),
            Token::Literal(Literal::IntegerLiteral(1)),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Comment(Comment::LineComment),
            Token::Comment(Comment::BlockComment),
            Token::EndOfFile,
        ];
        assert_tokens_match(input, expected);
    }

    #[test]
    fn test_complex_expression() {
        let input = r#"
fn main() -> Int {
    let result = (10 + 20) * 5; // Calculate result
    if result >= 100 && true {
        return "Success";
    } else if result < 100 {
        return "Failure";
    }
    /*
     * Some other logic
     */
    let a = "値"; // Multi-byte identifier and string (assuming backticks are part of identifier syntax, though dummy doesn't specifically handle backticks, it will parse as a regular identifier for "多言語")
    Type::new();
}
"#;
        let expected = vec![
            Token::Keyword(Keyword::Fn),
            Token::Identifier("main"),
            Token::Delimiter(Delimiter::LeftParen),
            Token::Delimiter(Delimiter::RightParen),
            Token::Operator(Operator::Arrow),
            Token::Keyword(Keyword::Int),
            Token::Delimiter(Delimiter::LeftBrace),
            Token::Keyword(Keyword::Let),
            Token::Identifier("result"),
            Token::Operator(Operator::Assignment),
            Token::Delimiter(Delimiter::LeftParen),
            Token::Literal(Literal::IntegerLiteral(10)),
            Token::Operator(Operator::Add),
            Token::Literal(Literal::IntegerLiteral(20)),
            Token::Delimiter(Delimiter::RightParen),
            Token::Operator(Operator::Multiply),
            Token::Literal(Literal::IntegerLiteral(5)),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Comment(Comment::LineComment),
            Token::Keyword(Keyword::If),
            Token::Identifier("result"),
            Token::Operator(Operator::GreaterThanOrEqual),
            Token::Literal(Literal::IntegerLiteral(100)),
            Token::Operator(Operator::LogicalAnd),
            Token::Literal(Literal::BoolLiteral(true)),
            Token::Delimiter(Delimiter::LeftBrace),
            Token::Keyword(Keyword::Return),
            Token::Literal(Literal::StringLiteral("Success")),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Delimiter(Delimiter::RightBrace),
            Token::Keyword(Keyword::Else),
            Token::Keyword(Keyword::If),
            Token::Identifier("result"),
            Token::Operator(Operator::LessThan),
            Token::Literal(Literal::IntegerLiteral(100)),
            Token::Delimiter(Delimiter::LeftBrace),
            Token::Keyword(Keyword::Return),
            Token::Literal(Literal::StringLiteral("Failure")),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Delimiter(Delimiter::RightBrace),
            Token::Comment(Comment::BlockComment),
            Token::Keyword(Keyword::Let),
            Token::Identifier("a"),
            Token::Operator(Operator::Assignment),
            Token::Literal(Literal::StringLiteral("値")),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Comment(Comment::LineComment), // Multi-byte identifier and string
            Token::Identifier("Type"),
            Token::Operator(Operator::NamespaceResolver),
            Token::Identifier("new"),
            Token::Delimiter(Delimiter::LeftParen),
            Token::Delimiter(Delimiter::RightParen),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Delimiter(Delimiter::RightBrace),
            Token::EndOfFile,
        ];
        assert_tokens_match(input, expected);
    }

    #[test]
    fn test_identifier_starting_with_keyword_prefix() {
        let input = "if_condition else_statement for_loop type_alias";
        let expected = vec![
            Token::Identifier("if_condition"),
            Token::Identifier("else_statement"),
            Token::Identifier("for_loop"),
            Token::Identifier("type_alias"),
            Token::EndOfFile,
        ];
        assert_tokens_match(input, expected);
    }

}
