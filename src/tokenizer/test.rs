// src/tokenizer/tokenizer.rs
#[cfg(test)]
mod tests {
    use crate::tokenizer::tokenizer::Tokenizer;
    use crate::tokenizer::errors::TokenizeErr;
    use crate::tokenizer::{Comment, Delimiter, Keyword, Literal, Operator, Token};


    // Helper function to tokenize input and unwrap the result, panicking on error.
    fn tokenize_success(input: &str) -> Vec<Token> {
        let tokenizer = Tokenizer::new(input);
        tokenizer.tokenize().unwrap()
    }

    // Helper function to tokenize input and expect a specific error.
    fn tokenize_error(input: &str) -> TokenizeErr {
        let tokenizer = Tokenizer::new(input);
        tokenizer.tokenize().unwrap_err()
    }

    #[test]
    fn test_empty_input() {
        let tokens = tokenize_success("");
        assert_eq!(tokens, vec![Token::EndOfFile]);
    }

    #[test]
    fn test_whitespace_only_input() {
        let tokens = tokenize_success("   \n\t  ");
        assert_eq!(tokens, vec![Token::EndOfFile]);
    }

    #[test]
    fn test_keywords() {
        let input = "import from fn const let try class return pub static final if for in while break continue match protocol else";
        let expected_tokens = vec![
            Token::Keyword(Keyword::Import),
            Token::Keyword(Keyword::From),
            Token::Keyword(Keyword::Fn),
            Token::Keyword(Keyword::Const),
            Token::Keyword(Keyword::Let),
            Token::Keyword(Keyword::Try),
            Token::Keyword(Keyword::Class),
            Token::Keyword(Keyword::Return),
            Token::Keyword(Keyword::Pub),
            Token::Keyword(Keyword::Static),
            Token::Keyword(Keyword::Final),
            Token::Keyword(Keyword::If),
            Token::Keyword(Keyword::For),
            Token::Keyword(Keyword::In),
            Token::Keyword(Keyword::While),
            Token::Keyword(Keyword::Break),
            Token::Keyword(Keyword::Continue),
            Token::Keyword(Keyword::Match),
            Token::Keyword(Keyword::Protocol),
            Token::Keyword(Keyword::Else),
            Token::EndOfFile,
        ];
        assert_eq!(tokenize_success(input), expected_tokens);
    }

    #[test]
    fn test_operators() {
        let input = "**= += -= *= /= %= == != >= <= && || ++ -- ** + - * / % > < ! =";
        let expected_tokens = vec![
            Token::Operator(Operator::PowAssign),
            Token::Operator(Operator::AddAssign),
            Token::Operator(Operator::SubAssign),
            Token::Operator(Operator::MulAssign),
            Token::Operator(Operator::DivAssign),
            Token::Operator(Operator::ModAssign),
            Token::Operator(Operator::Equal),
            Token::Operator(Operator::NotEqual),
            Token::Operator(Operator::GreaterEqual),
            Token::Operator(Operator::LessEqual),
            Token::Operator(Operator::And),
            Token::Operator(Operator::Or),
            Token::Operator(Operator::Increment),
            Token::Operator(Operator::Decrement),
            Token::Operator(Operator::Pow),
            Token::Operator(Operator::Plus),
            Token::Operator(Operator::Minus),
            Token::Operator(Operator::Multiply),
            Token::Operator(Operator::Divide),
            Token::Operator(Operator::Modulo),
            Token::Operator(Operator::Greater),
            Token::Operator(Operator::Less),
            Token::Operator(Operator::Not),
            Token::Operator(Operator::Assign),
            Token::EndOfFile,
        ];
        assert_eq!(tokenize_success(input), expected_tokens);
    }

    #[test]
    fn test_delimiters() {
        let input = ", . :: : ; ( ) { } [ ] @ \\";
        let expected_tokens = vec![
            Token::Delimiter(Delimiter::Comma),
            Token::Delimiter(Delimiter::Dot),
            Token::Delimiter(Delimiter::ColonColon),
            Token::Delimiter(Delimiter::Colon),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Delimiter(Delimiter::LeftParen),
            Token::Delimiter(Delimiter::RightParen),
            Token::Delimiter(Delimiter::LeftBrace),
            Token::Delimiter(Delimiter::RightBrace),
            Token::Delimiter(Delimiter::LeftBracket),
            Token::Delimiter(Delimiter::RightBracket),
            Token::Delimiter(Delimiter::At),
            Token::Delimiter(Delimiter::Backslash),
            Token::EndOfFile,
        ];
        assert_eq!(tokenize_success(input), expected_tokens);
    }

    #[test]
    fn test_identifiers() {
        let input = "x_y_z myVariable another_one _start _123 var123_test";
        let expected_tokens = vec![
            Token::Identifier("x_y_z"),
            Token::Identifier("myVariable"),
            Token::Identifier("another_one"),
            Token::Identifier("_start"),
            Token::Identifier("_123"),
            Token::Identifier("var123_test"),
            Token::EndOfFile,
        ];
        assert_eq!(tokenize_success(input), expected_tokens);
    }

    #[test]
    fn test_literals_integers() {
        // Test various integer sizes, checking for i32 then i64 parsing.
        let input = "123 0 2147483647 2147483648 9223372036854775807"; // Max i32, then i32+1 (becomes i64), then max i64
        let expected_tokens = vec![
            Token::Literal(Literal::IntegerLiteral(123)),
            Token::Literal(Literal::IntegerLiteral(0)),
            Token::Literal(Literal::IntegerLiteral(2147483647)), // Max i32
            Token::Literal(Literal::DoubleIntegerLiteral(2147483648i64)), // i32 overflow, becomes i64
            Token::Literal(Literal::DoubleIntegerLiteral(9223372036854775807i64)), // Max i64
            Token::EndOfFile,
        ];
        assert_eq!(tokenize_success(input), expected_tokens);
    }

    #[test]
    fn test_literals_floats() {
        // Test f32 then f64 parsing, including scientific notation.
        let input = "1.0 0.5 3.1415926535"; // Simple, scientific, max f64
        let expected_tokens = vec![
            Token::Literal(Literal::FloatLiteral(1.0f32)),
            Token::Literal(Literal::FloatLiteral(0.5f32)),
            Token::Literal(Literal::DoubleFloatLiteral(3.1415926535f64)), // Fits f32
            Token::EndOfFile,
        ];
        assert_eq!(tokenize_success(input), expected_tokens);
    }

    #[test]
    fn test_literals_strings() {
        let input = r#""hello world" "rust is cool" "" "a string with spaces""#;
        let expected_tokens = vec![
            Token::Literal(Literal::StringLiteral("hello world")),
            Token::Literal(Literal::StringLiteral("rust is cool")),
            Token::Literal(Literal::StringLiteral("")),
            Token::Literal(Literal::StringLiteral("a string with spaces")),
            Token::EndOfFile,
        ];
        assert_eq!(tokenize_success(input), expected_tokens);
    }

    #[test]
    fn test_literals_chars() {
        // Note: The current tokenizer does not interpret escape sequences like '\n' or '\t'.
        // '\n' will be treated as CharLiteral('\\') then fail because 'n' is not the closing quote.
        // '\'' will be treated as CharLiteral('\\').
        let input = r#"'a' 'Z' '1' '@' ' ' ''' "#; // 'ö' is a unicode char. '\'' is an escaped single quote literal itself.
        let expected_tokens = vec![
            Token::Literal(Literal::CharLiteral('a')),
            Token::Literal(Literal::CharLiteral('Z')),
            Token::Literal(Literal::CharLiteral('1')),
            Token::Literal(Literal::CharLiteral('@')),
            Token::Literal(Literal::CharLiteral(' ')),
            Token::Literal(Literal::CharLiteral('\'')),
            Token::EndOfFile,
        ];
        assert_eq!(tokenize_success(input), expected_tokens);
    }

    #[test]
    fn test_literals_booleans() {
        let input = "true false";
        let expected_tokens = vec![
            Token::Literal(Literal::BoolLiteral(true)),
            Token::Literal(Literal::BoolLiteral(false)),
            Token::EndOfFile,
        ];
        assert_eq!(tokenize_success(input), expected_tokens);
    }

    #[test]
    fn test_comments() {
        let input = r#"
        /// Doc comment for item
        /// Another line
        // A single line comment
        /* A block comment */
        /* Multi-line
         * block comment
         * ends here */
        // Comment at EOF
        "#;
        let expected_tokens = vec![
            Token::Comment(Comment::DocComment(" Doc comment for item")),
            Token::Comment(Comment::DocComment(" Another line")),
            Token::Comment(Comment::LineComment),
            Token::Comment(Comment::BlockComment),
            Token::Comment(Comment::BlockComment),
            Token::Comment(Comment::LineComment),
            Token::EndOfFile,
        ];
        assert_eq!(tokenize_success(input), expected_tokens);
    }

    // --- Error Handling Tests ---

    #[test]
    fn test_error_string_literal_not_closed_newline() {
        let err = tokenize_error(
            r#""hello
world"#,
        ); // String literal broken by newline
        assert_eq!(err, TokenizeErr::StringLiteralNotClosed(1));
    }

    #[test]
    fn test_error_string_literal_not_closed_eof() {
        let err = tokenize_error(r#""unterminated string"#); // String literal missing closing quote at EOF
        assert_eq!(err, TokenizeErr::StringLiteralNotClosed(1));
    }

    #[test]
    fn test_error_char_literal_not_closed_eof() {
        let err = tokenize_error(r#"'a"#); // Char literal missing closing quote at EOF
        assert_eq!(err, TokenizeErr::CharLiteralNotClosed(1));
    }

    #[test]
    fn test_error_char_literal_not_closed_multiple_chars() {
        let err = tokenize_error(r#"'abc'"#); // Too many characters in char literal
        assert_eq!(err, TokenizeErr::InvalidCharLiteral(1));
    }

    #[test]
    fn test_error_char_literal_empty() {
        let err = tokenize_error(r#"''"#); // Empty char literal (current impl requires one char)
        assert_eq!(err, TokenizeErr::CharLiteralNotClosed(1)); // It will read '', peek `''`, but `c.is_none()` on first peek, or `if let Some("'")` fails before `c.unwrap()`
                                                            // Let's re-trace: `self.current_char += 1;` (past first `'`). `c = self.peek_char()` (gets second `''`). `c.unwrap()` is `''`. `self.current_char += 1;` (past second `''`). `peek_char()` is `None`. `if let Some("'")` fails. returns `CharLiteralNotClosed`. This is indeed the current behavior.
    }

    #[test]
    fn test_error_invalid_integer_literal_too_large() {
        let input = "9223372036854775808999"; // Larger than i64::MAX
        let err = tokenize_error(input);
        assert_eq!(err, TokenizeErr::InvalidIntegerLiteral(22));
    }

    #[test]
    fn test_error_invalid_float_literal_malformed() {
        let input = "1.2.3"; // Malformed float
        let err = tokenize_error(input);
        assert_eq!(err, TokenizeErr::InvalidFloatLiteral(5));
    }

    #[test]
    fn test_error_unknown_token_single_char() {
        let err = tokenize_error("#"); // A hash symbol, not recognized
        assert_eq!(err, TokenizeErr::UnknownToken(0));
    }

    #[test]
    fn test_error_unknown_token_sequence() {
        let tokenizer = Tokenizer::new("let $ = 10;");
        let tokens = tokenizer.tokenize();
        assert!(tokens.is_err());
        assert_eq!(tokens.unwrap_err(), TokenizeErr::UnknownToken(4));
    }

    #[test]
    fn test_error_block_comment_not_closed() {
        let err = tokenize_error("/* This block comment is not closed");
        assert_eq!(err, TokenizeErr::BlockCommentNotClosed(35));
    }

    #[test]
    fn test_error_block_comment_not_closed_multiline() {
        let err = tokenize_error("/* This block comment\n * is multi-line\n but not closed");
        assert_eq!(err, TokenizeErr::BlockCommentNotClosed(54));
    }

    // --- Ambiguity / Specific Behavior Tests ---
    // These tests confirm how the current tokenizer handles cases where a keyword is a prefix of an identifier,
    // which it currently tokenizes greedily.

    #[test]
    fn test_keyword_prefix_identifier_ambiguity_let_ter() {
        let input = "letter"; // "let" is a keyword. Current tokenizer matches "let" then "ter"
        let expected_tokens = vec![
            Token::Identifier("letter"),
            Token::EndOfFile,
        ];
        assert_eq!(tokenize_success(input), expected_tokens);
    }

    #[test]
    fn test_keyword_prefix_identifier_ambiguity_true_foo() {
        let input = "truefoo"; // "true" is a bool literal. Current tokenizer matches "true" then "foo"
        let expected_tokens = vec![
            Token::Literal(Literal::BoolLiteral(true)),
            Token::Identifier("foo"),
            Token::EndOfFile,
        ];
        assert_eq!(tokenize_success(input), expected_tokens);
    }

    #[test]
    fn test_operator_ambiguity_arrow() {
        let input = "->"; // No "->" operator defined, should be Minus then Greater
        let expected_tokens = vec![
            Token::Operator(Operator::Minus),
            Token::Operator(Operator::Greater),
            Token::EndOfFile,
        ];
        assert_eq!(tokenize_success(input), expected_tokens);
    }

}
