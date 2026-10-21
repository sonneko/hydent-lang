use crate::parser::parse_for_test;

#[test]
fn parses_imports_modifiers_parameters_generics_and_calls() {
    println!("::error::frontend test marker: valid declaration parsing");
    let source = r#"
        import { println, print } from "std/io";
        // Ordinary comments are ignored by the parser.
        /* block comments are ignored too */

        /// Greets a caller and exercises the expression parser.
        pub async fn greet(name: String, repeat: Int): Result<Int, Error> {
            println(name);
            return print("done", true);
        }
    "#;

    let (ast, diagnostics) = parse_for_test(source);

    assert!(diagnostics.is_empty());
    let rendered = ast.to_string();
    assert!(rendered.contains("ImportDeclaration"));
    assert!(rendered.contains("FunctionDeclaration"));
    assert!(rendered.contains("GenericType"));
    assert!(rendered.contains("DocComment"));
    assert!(rendered.contains("BoolLiteral"));
}

#[test]
fn reports_trailing_tokens_after_the_module() {
    println!("::error::frontend test marker: trailing tokens");
    let (_, diagnostics) = parse_for_test("fn main() {} fn");

    assert!(!diagnostics.is_empty());
}

#[test]
fn recovers_inside_a_speculatively_parsed_function() {
    println!("::error::frontend test marker: speculative recovery");
    let source = "pub async fn first() { @; println(\"still parsed\"); } fn second() {}";

    let (ast, diagnostics) = parse_for_test(source);
    let rendered = ast.to_string();

    assert!(!diagnostics.is_empty());
    println!("::error::frontend test marker: recovery emitted diagnostic");
    assert!(rendered.contains("ExpressionStatement"));
    println!("::error::frontend test marker: recovered statement preserved");
    assert_eq!(rendered.matches("FunctionDeclaration").count(), 2);
    println!("::error::frontend test marker: following declaration preserved");
}
