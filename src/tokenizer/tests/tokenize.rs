
use crate::compiler::source_holder::SourceHolder;
use crate::compiler::symbol::SymbolFactory;
use crate::tokenizer::tokenize::Tokenizer;
use crate::tokenizer::tokens::{Comment, Delimiter, Keyword, Literal, Operator, Token};

/// テスト用のヘルパー関数。
/// 文字列を入力として受け取り、トークン列を返す。
fn tokenize_helper(input: &str) -> Vec<Token> {
    let source_holder = SourceHolder::new(input);
    let mut symbol_factory = SymbolFactory::new(source_holder);
    let tokenizer = Tokenizer::new(input, &mut symbol_factory);
    tokenizer.tokenize().expect("Tokenization failed")
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
    // 整数、浮動小数点、16進数、2進数、科学表記
    let input = "123 3.14 0xFF 0b1010 1.2e+10";
    let tokens = tokenize_helper(input);

    assert!(matches!(
        tokens[0],
        Token::Literal(Literal::IntegerLiteral(123))
    ));
    // f32の比較はHashableFloatを介して行われる
    if let Token::Literal(Literal::FloatLiteral(f)) = tokens[1] {
        assert_eq!(f.get(), 3.14);
    } else {
        panic!("Expected float");
    }

    assert!(matches!(
        tokens[2],
        Token::Literal(Literal::IntegerLiteral(255))
    )); // 0xFF
    assert!(matches!(
        tokens[3],
        Token::Literal(Literal::IntegerLiteral(10))
    )); // 0b1010

    if let Token::Literal(Literal::FloatLiteral(f)) = tokens[4] {
        assert_eq!(f.get(), 1.2e10);
    } else {
        panic!("Expected float scientific");
    }
}

#[test]
fn test_operators_and_ranges() {
    // 最大長のものから優先的にマッチするか(..= vs .. vs .)
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
    assert_eq!(tokens[3], Token::Comment(Comment::BlockComment)); // ネストされたものも1つのBlockCommentとして処理
}

#[test]
fn test_multibyte_safety_in_comments_and_strings() {
    // 日本語（3バイト文字）や絵文字（4バイト文字）を含むケース
    // Tokenizerの各メソッドがバイトインデックスではなく文字境界を意識できているか確認
    let input = r#"
            // 日本語のコメント
            let s = "こんにちは、世界 🌍"; 
            /* マルチバイト
               ブロックコメント */
            let c = 'あ';
        "#;

    // tokenize中にパニック（境界外アクセスや不正なUTF-8スライス作成）が起きないことを確認
    let tokens = tokenize_helper(input);

    assert_eq!(tokens[0], Token::Comment(Comment::LineComment));
    assert_eq!(tokens[1], Token::Keyword(Keyword::Let));
    assert!(tokens[2].is_identifier());
    assert_eq!(tokens[3], Token::Operator(Operator::Assignment));

    // 文字列リテラルの中身が正しくSpanとして認識されているか
    if let Token::Literal(Literal::StringLiteral(span)) = tokens[4] {
        // 文字列自体のバリデーションはParser/Evaluatorの責務だが、
        // 終端の引用符が正しく認識されているかが重要
        assert_eq!(tokens[5], Token::Delimiter(Delimiter::Semicolon));
    } else {
        panic!("Expected string literal");
    }

    assert_eq!(tokens[6], Token::Comment(Comment::BlockComment));
    assert_eq!(tokens[11], Token::Literal(Literal::CharLiteral('あ')));
}

#[test]
fn test_multibyte_error_handling() {
    // 識別子として許可されていないマルチバイト文字が直接現れた場合
    let input = "let 🍕 = 1;";
    let source_holder = SourceHolder::new(input);
    let mut symbol_factory = SymbolFactory::new(source_holder);
    let tokenizer = Tokenizer::new(input, &mut symbol_factory);

    let result = tokenizer.tokenize();
    // 現在の実装では UnknownToken エラーになるはず
    assert!(result.is_err());
}

#[test]
fn test_number_literal_after_multibyte() {
    // read_number_literal 内の unsafe { std::str::from_utf8_unchecked }
    // が直前のマルチバイト文字の影響で不正なポインタを参照しないか
    let input = "// あ\n123";
    let tokens = tokenize_helper(input);

    // コメント（マルチバイト含む）の後の数値が正しく読み取れるか
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
        // 正常にクローズされている
        assert_eq!(tokens[1], Token::EndOfFile);
    } else {
        panic!("String literal with escapes failed");
    }
}

#[test]
fn test_incomplete_tokens() {
    let source_holder = SourceHolder::new("\"unclosed string");
    let mut symbol_factory = SymbolFactory::new(source_holder);

    // 閉じられていない文字列リテラル
    let tokenizer1 = Tokenizer::new("\"unclosed string", &mut symbol_factory);
    assert!(tokenizer1.tokenize().is_err());

    // 閉じられていないブロックコメント
    let tokenizer2 = Tokenizer::new("/* unclosed comment", &mut symbol_factory);
    assert!(tokenizer2.tokenize().is_err());

    // 閉じられていない文字リテラル
    let tokenizer3 = Tokenizer::new("'a", &mut symbol_factory);
    assert!(tokenizer3.tokenize().is_err());
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
