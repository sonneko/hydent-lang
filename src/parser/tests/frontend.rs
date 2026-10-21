use crate::parser::parse_for_test;

#[test]
fn parses_imports_modifiers_parameters_generics_and_calls() {
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
    let (_, diagnostics) = parse_for_test("fn main() {} fn");

    assert!(!diagnostics.is_empty());
}

#[test]
fn recovers_at_a_later_top_level_declaration() {
    let (ast, diagnostics) = parse_for_test("fn broken {} fn kept() {}");
    let rendered = ast.to_string();

    assert!(!diagnostics.is_empty());
    assert!(rendered.contains(r#""kept""#));
    assert!(!rendered.contains(r#""broken""#));
}

#[test]
fn recovers_after_a_failed_top_level_speculation() {
    let source = "pub async fn first { @; println(\"still parsed\"); } fn second() {}";

    let (ast, diagnostics) = parse_for_test(source);
    let rendered = ast.to_string();

    assert!(!diagnostics.is_empty());
    assert!(rendered.contains("RawDeclaration"));
    assert!(rendered.contains(r#""second""#));
    assert_eq!(
        rendered.matches("\"kind\":\"FunctionDeclaration\"").count(),
        1
    );
}
