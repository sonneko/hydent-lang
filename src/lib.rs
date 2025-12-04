mod tokenizer;
mod parser;
mod common;
mod error;

pub use tokenizer::tokenizer::Tokenizer;
pub use parser::parser::Parser;

use crate::common::symbol::SymbolFactory;

pub fn run_frontend(program: &str) -> Result<(), String> {
    let mut symbol_factory = SymbolFactory::new(program);
    let tokenizer = Tokenizer::new(program, &mut symbol_factory);
    let tokens = tokenizer.tokenize().unwrap();
    let tokens_iter = tokens.into_iter();
    let ast = Parser::new(tokens_iter.peekable()).parse().unwrap();

    println!("{:?}", ast);

    unimplemented!();
}