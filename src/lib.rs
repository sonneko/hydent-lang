mod tokenizer;
mod parser;

pub use tokenizer::tokenizer::Tokenizer;
pub use parser::parser::Parser;

pub fn compile(program: &str) -> Result<(), String> {
    let tokenizer = Tokenizer::new(program);
    let tokens = tokenizer.tokenize().unwrap();
    
    let ast = Parser::new(&tokens).parse().unwrap();

    println!("{:?}", ast);

    unimplemented!();
}