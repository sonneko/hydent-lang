mod tokenizer;
pub use tokenizer::tokenizer::Tokenizer;

pub fn compile(program: &str) -> Result<(), String> {
    let tokenizer = Tokenizer::new(program);
    let tokens = tokenizer.tokenize();
    println!("{:?}", tokens);

    unimplemented!();
}