#[cfg(test)]
mod tests {
    use crate::tokenizer::{Token, Keyword, Literal, Operator, Comment, Delimiter};
    use crate::tokenizer::errors::TokenizeErr;
    use crate::Tokenizer;

    // ヘルパー関数: トークナイザを実行し、結果を返します
    fn get_tokens(input: &str) -> Result<Vec<Token<'_>>, TokenizeErr> {
        let tokenizer = Tokenizer::new(input);
        tokenizer.tokenize()
    }

    // --- 基本的なケース ---

    #[test]
    fn test_empty_input() {
        let tokens = get_tokens("").unwrap();
        assert_eq!(tokens, vec![Token::EndOfFile]);
    }

    #[test]
    fn test_whitespace_only_input() {
        let tokens = get_tokens("   \t\n\r ").unwrap();
        assert_eq!(tokens, vec![Token::EndOfFile]);
    }

    // --- キーワードのテスト ---

    #[test]
    fn test_keywords() {
        let input = "import from fn const let try class return pub static final if else for in while break continue match protocol";
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
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
                Token::Keyword(Keyword::Else),
                Token::Keyword(Keyword::For),
                Token::Keyword(Keyword::In),
                Token::Keyword(Keyword::While),
                Token::Keyword(Keyword::Break),
                Token::Keyword(Keyword::Continue),
                Token::Keyword(Keyword::Match),
                Token::Keyword(Keyword::Protocol),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_keyword_as_identifier_prefix() {
        // キーワードが識別子のプレフィックスになっている場合のテスト (例: "let_x" は "let" ではない)
        let tokens = get_tokens("let_x").unwrap();
        assert_eq!(tokens, vec![Token::Identifier("let_x"), Token::EndOfFile]);
    }

    // --- 識別子のテスト ---

    #[test]
    fn test_identifiers() {
        let input = "variable_name _private funcName CONSTANT";
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("variable_name"),
                Token::Identifier("_private"),
                Token::Identifier("funcName"),
                Token::Identifier("CONSTANT"),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_identifier_with_digits() {
        let tokens = get_tokens("v1riable func234").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("v1riable"),
                Token::Identifier("func234"),
                Token::EndOfFile,
            ]
        );
    }

    // --- リテラルのテスト ---

    #[test]
    fn test_integer_literals() {
        let input = "123 0 -456";
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::IntegerLiteral(123)),
                Token::Literal(Literal::IntegerLiteral(0)),
                Token::Operator(Operator::Minus), // -456は '-' と '456' に分割されるはず
                Token::Literal(Literal::IntegerLiteral(456)),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_double_integer_literals() {
        let input = "123 9223372036854775807";
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::IntegerLiteral(123)),
                Token::Literal(Literal::DoubleIntegerLiteral(9223372036854775807)),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_float_literals() {
        // TODO: implement 11.2e+11 like format
        let input = "1.23 0.0 ";
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::FloatLiteral(1.23)),
                Token::Literal(Literal::FloatLiteral(0.0)),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_double_float_literals() {
        let input = "1.2334242323 3.1415926535";
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::DoubleFloatLiteral(1.2334242323)),
                Token::Literal(Literal::DoubleFloatLiteral(3.1415926535)),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_string_literals() {
        // TODO: implement Raw string to simplify escaping
        let input = r#""hello" "world!" "#; 
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::StringLiteral("hello")),
                Token::Literal(Literal::StringLiteral("world!")),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_char_literals() {
        // TODO: implement for '\n' or '\t'
        let input = r#"'a' 'Z' '1' '€'"#;
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::CharLiteral('a')),
                Token::Literal(Literal::CharLiteral('Z')),
                Token::Literal(Literal::CharLiteral('1')),
                // Token::Literal(Literal::CharLiteral('\'')),
                // Token::Literal(Literal::CharLiteral('\n')),
                // Token::Literal(Literal::CharLiteral('\t')),
                // Token::Literal(Literal::CharLiteral('\\')),
                Token::Literal(Literal::CharLiteral('€')), // UTF-8 char
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_bool_literals() {
        let input = "true false";
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::BoolLiteral(true)),
                Token::Literal(Literal::BoolLiteral(false)),
                Token::EndOfFile,
            ]
        );
    }

    // --- 演算子のテスト ---

    #[test]
    fn test_operators() {
        let input = "+ - * ** / % += -= *= **= /= %= == != > < >= <= && || ! ++ -- .. =";
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Operator(Operator::Plus),
                Token::Operator(Operator::Minus),
                Token::Operator(Operator::Multiply),
                Token::Operator(Operator::Pow),
                Token::Operator(Operator::Divide),
                Token::Operator(Operator::Modulo),
                Token::Operator(Operator::AddAssign),
                Token::Operator(Operator::SubAssign),
                Token::Operator(Operator::MulAssign),
                Token::Operator(Operator::PowAssign),
                Token::Operator(Operator::DivAssign),
                Token::Operator(Operator::ModAssign),
                Token::Operator(Operator::Equal),
                Token::Operator(Operator::NotEqual),
                Token::Operator(Operator::Greater),
                Token::Operator(Operator::Less),
                Token::Operator(Operator::GreaterEqual),
                Token::Operator(Operator::LessEqual),
                Token::Operator(Operator::And),
                Token::Operator(Operator::Or),
                Token::Operator(Operator::Not),
                Token::Operator(Operator::Increment),
                Token::Operator(Operator::Decrement),
                Token::Operator(Operator::CommaComma),
                Token::Operator(Operator::Assign),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_operator_precedence_like_tokenization() {
        // 例えば、`=`の前に`==`が正しく認識されるか
        let tokens = get_tokens("===!").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Operator(Operator::Equal),
                Token::Operator(Operator::Assign),
                Token::Operator(Operator::Not),
                Token::EndOfFile,
            ]
        );
        // `>=`と`>`
        let tokens = get_tokens(">= >").unwrap();
        assert_eq!(tokens, vec![Token::Operator(Operator::GreaterEqual), Token::Operator(Operator::Greater), Token::EndOfFile]);
    }

    // --- コメントのテスト ---

    #[test]
    fn test_line_comments() {
        let input = r#"
// This is a line comment
let x = 10; // Inline comment
"#;
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Comment(Comment::LineComment),
                Token::Keyword(Keyword::Let),
                Token::Identifier("x"),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::IntegerLiteral(10)),
                Token::Delimiter(Delimiter::Semicolon),
                Token::Comment(Comment::LineComment),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_doc_comments() {
        let input = r#"
/// This is a doc comment
/// Another line of doc comment
fn func() {}
"#;
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Comment(Comment::DocComment(" This is a doc comment")), // 内容もキャプチャされることを期待
                Token::Comment(Comment::DocComment(" Another line of doc comment")),
                Token::Keyword(Keyword::Fn),
                Token::Identifier("func"),
                Token::Delimiter(Delimiter::LeftParen),
                Token::Delimiter(Delimiter::RightParen),
                Token::Delimiter(Delimiter::LeftBrace),
                Token::Delimiter(Delimiter::RightBrace),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_block_comments() {
        let input = r#"
/* This is a block comment */
/*
 * Multi-line block comment
 * with stars
 */
let y = 20;
"#;
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Comment(Comment::BlockComment), // 内容はキャプチャされない想定
                Token::Comment(Comment::BlockComment),
                Token::Keyword(Keyword::Let),
                Token::Identifier("y"),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::IntegerLiteral(20)),
                Token::Delimiter(Delimiter::Semicolon),
                Token::EndOfFile,
            ]
        );
    }


    // --- デリミタのテスト ---

    #[test]
    fn test_delimiters() {
        let input = ", . : :: ; ( ) { } [ ] @ \\ #";
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Delimiter(Delimiter::Comma),
                Token::Delimiter(Delimiter::Dot),
                Token::Delimiter(Delimiter::Colon),
                Token::Delimiter(Delimiter::ColonColon),
                Token::Delimiter(Delimiter::Semicolon),
                Token::Delimiter(Delimiter::LeftParen),
                Token::Delimiter(Delimiter::RightParen),
                Token::Delimiter(Delimiter::LeftBrace),
                Token::Delimiter(Delimiter::RightBrace),
                Token::Delimiter(Delimiter::LeftBracket),
                Token::Delimiter(Delimiter::RightBracket),
                Token::Delimiter(Delimiter::At),
                Token::Delimiter(Delimiter::Backslash),
                Token::Delimiter(Delimiter::Sharp),
                Token::EndOfFile,
            ]
        );
    }

    // --- 複合的なコードスニペットのテスト ---

    #[test]
    fn test_complex_snippet_1() {
        let input = "fn main() { let x = 1 + 2; println(\"hello\"); }";
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::Fn),
                Token::Identifier("main"),
                Token::Delimiter(Delimiter::LeftParen),
                Token::Delimiter(Delimiter::RightParen),
                Token::Delimiter(Delimiter::LeftBrace),
                Token::Keyword(Keyword::Let),
                Token::Identifier("x"),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::IntegerLiteral(1)),
                Token::Operator(Operator::Plus),
                Token::Literal(Literal::IntegerLiteral(2)),
                Token::Delimiter(Delimiter::Semicolon),
                Token::Identifier("println"),
                Token::Delimiter(Delimiter::LeftParen),
                Token::Literal(Literal::StringLiteral("hello")),
                Token::Delimiter(Delimiter::RightParen),
                Token::Delimiter(Delimiter::Semicolon),
                Token::Delimiter(Delimiter::RightBrace),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_complex_snippet_2_if_else() {
        let input = "if (x > 0) { return true; } else { return false; }";
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::If),
                Token::Delimiter(Delimiter::LeftParen),
                Token::Identifier("x"),
                Token::Operator(Operator::Greater),
                Token::Literal(Literal::IntegerLiteral(0)),
                Token::Delimiter(Delimiter::RightParen),
                Token::Delimiter(Delimiter::LeftBrace),
                Token::Keyword(Keyword::Return),
                Token::Literal(Literal::BoolLiteral(true)),
                Token::Delimiter(Delimiter::Semicolon),
                Token::Delimiter(Delimiter::RightBrace),
                Token::Keyword(Keyword::Else),
                Token::Delimiter(Delimiter::LeftBrace),
                Token::Keyword(Keyword::Return),
                Token::Literal(Literal::BoolLiteral(false)),
                Token::Delimiter(Delimiter::Semicolon),
                Token::Delimiter(Delimiter::RightBrace),
                Token::EndOfFile,
            ]
        );
    }

    // --- エラーケースのテスト ---

    #[test]
    fn test_error_string_literal_not_closed() {
        let err = get_tokens(r#""unclosed string"#).unwrap_err();
        assert_eq!(err, TokenizeErr::StringLiteralNotClosed(1));
    }

    #[test]
    fn test_error_char_literal_not_closed() {
        let err = get_tokens(r#"'a"#).unwrap_err();
        assert_eq!(err, TokenizeErr::CharLiteralNotClosed(1));
    }

    #[test]
    fn test_error_invalid_char_literal_empty() {
        let err = get_tokens(r#"''"#).unwrap_err(); // 空の文字リテラル
        assert_eq!(err, TokenizeErr::InvalidCharLiteral(1));
    }

    #[test]
    fn test_error_invalid_char_literal_too_long() {
        let err = get_tokens(r#"'abc'"#).unwrap_err(); // 複数文字
        assert_eq!(err, TokenizeErr::InvalidCharLiteral(1));
    }

    // #[test]
    // fn test_error_invalid_char_literal_bad_escape() {
    //     let err = get_tokens(r#"'\z'"#).unwrap_err(); // 不正なエスケープシーケンス
    //     assert_eq!(err, TokenizeErr::InvalidCharLiteral(1));
    // }

    #[test]
    fn test_error_invalid_integer_literal_overflow() {
        // i64の最大値を超える数値
        let err = get_tokens("214748364342324232324232423422348").unwrap_err(); // 2^31
        assert_eq!(err, TokenizeErr::InvalidIntegerLiteral(33));
    }

    #[test]
    fn test_error_invalid_integer_literal_malformed() {
        let err = get_tokens("123abc").unwrap();
        assert_eq!(err, vec![
            Token::Literal(Literal::IntegerLiteral(123)),
            Token::Identifier("abc"),
            Token::EndOfFile,
        ]);
    }

    #[test]
    fn test_error_invalid_float_literal_malformed() {
        let err = get_tokens("1.2.3").unwrap_err();
        assert_eq!(err, TokenizeErr::InvalidFloatLiteral(5)); // 2番目のドットが不正
    }

    #[test]
    fn test_error_unknown_token() {
        let err = get_tokens("$unknown").unwrap_err();
        assert_eq!(err, TokenizeErr::UnknownToken(0));
    }

    #[test]
    fn test_error_block_comment_not_closed() {
        let err = get_tokens("/* unclosed comment").unwrap_err();
        assert_eq!(err, TokenizeErr::BlockCommentNotClosed(19));
    }

    // --- 空白文字と改行の処理 ---

    #[test]
    fn test_whitespace_around_tokens() {
        let input = "  let   x = \t 10 ; \n";
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::Let),
                Token::Identifier("x"),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::IntegerLiteral(10)),
                Token::Delimiter(Delimiter::Semicolon),
                Token::EndOfFile,
            ]
        );
    }

    // --- マルチバイト文字のテスト ---

    #[test]
    fn test_multibyte_characters_in_string_literal() {
        // UTF-8文字列リテラルが正しく扱われるか
        let input = r#""こんにちは世界！😊""#;
        let tokens = get_tokens(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::StringLiteral("こんにちは世界！😊")),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_multibyte_char_as_unknown_token() {
        // 識別子として許可されない場合、未知のトークンとして扱われるべき
        let err = get_tokens("$").unwrap_err();
        assert_eq!(err, TokenizeErr::UnknownToken(0));
    }

    // --- EOFのテスト ---
    #[test]
    fn test_eof_after_tokens() {
        let tokens = get_tokens("1 + 2").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::IntegerLiteral(1)),
                Token::Operator(Operator::Plus),
                Token::Literal(Literal::IntegerLiteral(2)),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn test_eof_after_whitespace_at_end() {
        let tokens = get_tokens("1 + 2   ").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Literal(Literal::IntegerLiteral(1)),
                Token::Operator(Operator::Plus),
                Token::Literal(Literal::IntegerLiteral(2)),
                Token::EndOfFile,
            ]
        );
    }
}
