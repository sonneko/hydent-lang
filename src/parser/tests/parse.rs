use crate::{
    compiler::{
        arena::Arena, context::frontend::CompilerFrontendContext, source_holder::SourceHolder,
        symbol::SymbolFactory,
    },
    parser::{
        ast::{ASTVisitor, Module},
        errors::ParseErr,
        generated_ast_printer::ASTPrinter,
        parse::Parser,
        Ast,
    },
    tokenizer::{token_stream::TokenStream, tokenize::Tokenizer},
};

fn parse(source: &str) {
    let source = String::from(source);
    let mut symbols = SymbolFactory::new(SourceHolder::new(&source));
    let tokenizer = Tokenizer::new(&source, &mut symbols);
    let (tokens, errors) = tokenizer.tokenize();
    let stream = TokenStream::new(tokens);
    println!("{}", stream);
    let mut ast_arena = Arena::new();
    let mut errors_arena = Arena::new();
    let ast = {
        let mut parser = Parser::new(
            stream,
            CompilerFrontendContext {
                source: SourceHolder::new(&source),
                symbol_factory: &mut symbols,
                ast_arena: &mut ast_arena,
                errors_arena: &mut errors_arena,
            },
        );
        let ast = parser.parse();
        ast
    };
    println!("parsed.");
    let ast = Ast::new(ast.unwrap(), ast_arena, SourceHolder::new(&source), symbols);
    println!("{}", ast);
}

#[test]
fn test_parse() {
    parse(
        r#"
import { Result, Ok, Err } from "std/result";
import { Option, Some, None } from "std/option";

class UserError {
    message: String;

    try fn get_name(self): String {
        if self.name.is_empty() {
            panic("user is empty");
        }
        return self.name;
    }

    pub fn new(name: String): Result<Self, UserError> {
        if name.is_empty() {
            return Err(UserError { message: "name is empty" });
        }
        std::io::println("verified user");
        Ok(Self { name })
    }
}

fn main() {
    match User::new("Alice".to_string()) {
        Ok(user) => {
            let name = try user.get_name();
            std::io::println("created users" + name);
        },
        Err(error) => {
            std::io::println("fail to create users" + error.message);
        },
    }   
    "#,
    )
}

