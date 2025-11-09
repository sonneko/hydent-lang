mod tokenizer;

use tokenizer::tokenizer::Tokenizer;

fn main() {
    println!("Hello, world!");
    let tokenizer = Tokenizer::new("let x = 10;");
    let tokens = tokenizer.tokenize();
    println!("{:?}", tokens);
}
