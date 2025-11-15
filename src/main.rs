mod tokenizer;
mod parser;

use tokenizer::tokenizer::Tokenizer;

fn main() {
    let tokenizer = Tokenizer::new("let x = 10;");
    let tokens = tokenizer.tokenize(); // returns Result<Vec<Token<'_>>, TokenizeErr>
    println!("{:?}", tokens);
}
