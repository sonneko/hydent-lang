use crate::compiler::source_holder::SourceHolder;
use crate::compiler::symbol::SymbolFactory;
use crate::diagnostic::stream::IgnoreDiagnosticStream;
use crate::tokenizer::tokenize::Tokenizer;
use crate::tokenizer::tokens::{Comment, Delimiter, Keyword, Literal, Operator, Token};

fn tokenize_helper(input: &str) -> Vec<Token> {
    let mut diagnostic_stream = IgnoreDiagnosticStream::new();
    let mut symbol_factory = SymbolFactory::new(input);
    let tokenizer = Tokenizer::new(input, &mut symbol_factory);
    let (tokens, _) = tokenizer.tokenize(&mut diagnostic_stream);
    tokens.into_iter().map(|(token, _)| token).collect()
}

#[test]
fn test_basic_keywords() {
    let input = "let mut pub fn class if else match return";
    let tokens = tokenize_helper(input);

    assert_eq!(tokens[0], Token::Keyword(Keyword::Let));
    assert_eq!(tokens[1], Token::Keyword(Keyword::Mut));
    assert_eq!(tokens[2], Token::Keyword(Keyword::Pub));
    assert_eq!(tokens[3], Token::Keyword(Keyword::Fn));
    assert_eq!(tokens[4], Token::Keyword(Keyword::Class));
    assert_eq!(tokens[5], Token::Keyword(Keyword::If));
    assert_eq!(tokens[6], Token::Keyword(Keyword::Else));
    assert_eq!(tokens[7], Token::Keyword(Keyword::Match));
    assert_eq!(tokens[8], Token::Keyword(Keyword::Return));
    assert_eq!(tokens[9], Token::EndOfFile);
}

#[test]
fn test_numbers() {
    let input = "123 3.14 1.2e+10";
    let tokens = tokenize_helper(input);

    assert!(matches!(
        tokens[0],
        Token::Literal(Literal::IntegerLiteral(123))
    ));
    if let Token::Literal(Literal::FloatLiteral(f)) = tokens[1] {
        assert_eq!(f.get(), 3.14);
    } else {
        panic!("Expected float");
    }

    if let Token::Literal(Literal::FloatLiteral(f)) = tokens[2] {
        assert_eq!(f.get(), 1.2e10);
    } else {
        panic!("Expected float scientific");
    }
}

#[test]
fn test_operators_and_ranges() {
    let input = ".. ..= . => -> |>";
    let tokens = tokenize_helper(input);

    assert_eq!(tokens[0], Token::Operator(Operator::RangeExclusive));
    assert_eq!(tokens[1], Token::Operator(Operator::RangeInclusive));
    assert_eq!(tokens[2], Token::Operator(Operator::MemberAccess));
    assert_eq!(tokens[3], Token::Operator(Operator::FatArrow));
    assert_eq!(tokens[4], Token::Operator(Operator::Arrow));
    assert_eq!(tokens[5], Token::Operator(Operator::Pipe));
}

#[test]
fn test_comments() {
    let input = "// line comment\n/// doc comment\n/* block */ /* nested /* block */ */";
    let tokens = tokenize_helper(input);

    assert_eq!(tokens[0], Token::Comment(Comment::LineComment));
    assert!(matches!(tokens[1], Token::Comment(Comment::DocComment(_))));
    assert_eq!(tokens[2], Token::Comment(Comment::BlockComment));
    assert_eq!(tokens[3], Token::Comment(Comment::BlockComment));
}

#[test]
fn test_multibyte_safety_in_comments_and_strings() {
    // TODO: may be occure undefined behavior
    let input = r#"
            // 日本語のコメント
            let s = "こんにちは、世界 🌍"; 
            /* マルチバイト
               ブロックコメント */
            'あ'
        "#;

    let tokens = tokenize_helper(input);

    assert_eq!(tokens[0], Token::Comment(Comment::LineComment));
    assert_eq!(tokens[1], Token::Keyword(Keyword::Let));
    assert!(tokens[2].is_identifier());
    assert_eq!(tokens[3], Token::Operator(Operator::Assignment));

    if let Token::Literal(Literal::StringLiteral(span)) = tokens[4] {
        assert_eq!(tokens[5], Token::Delimiter(Delimiter::Semicolon));
    } else {
        panic!("Expected string literal");
    }

    assert_eq!(tokens[6], Token::Comment(Comment::BlockComment));
    assert_eq!(tokens[7], Token::Literal(Literal::CharLiteral('あ')));
}

#[test]
fn test_number_literal_after_multibyte() {
    let input = "// あ\n123";
    let tokens = tokenize_helper(input);

    assert_eq!(tokens[0], Token::Comment(Comment::LineComment));
    assert!(matches!(
        tokens[1],
        Token::Literal(Literal::IntegerLiteral(123))
    ));
}

#[test]
fn test_string_escape_sequences() {
    let input = r#""line 1\nline 2\"quoted\"""#;
    let tokens = tokenize_helper(input);

    if let Token::Literal(Literal::StringLiteral(_)) = tokens[0] {
        assert_eq!(tokens[1], Token::EndOfFile);
    } else {
        panic!("String literal with escapes failed");
    }
}

#[test]
fn test_complex_scientific_notation() {
    let input = "1.0e+10 1.0E-10 1e5";
    let tokens = tokenize_helper(input);

    for i in 0..3 {
        if let Token::Literal(Literal::FloatLiteral(_)) = tokens[i] {
            // ok
        } else {
            panic!("Scientific notation failed at index {}", i);
        }
    }
}
