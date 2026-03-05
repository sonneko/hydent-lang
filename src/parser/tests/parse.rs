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
        parse_for_integration_test, Ast,
    },
    tokenizer::{token_stream::TokenStream, tokenize::Tokenizer},
};

#[test]
fn test_parse() {
    let ast = parse_for_integration_test(
        r#"
import { Result, Ok, Err } from "std/result";
import { Option, Some, None } from "std/option";

class UserError {
    final message: String;

    pub fn get_name(self): String {
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
    );
    println!("{}", ast);
}
