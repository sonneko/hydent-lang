use crate::compiler::{query_sys::{Database, Query}, source_holder::SourceHolder, symbol::SymbolFactory};

mod test;
mod tokenizer;
mod errors;
pub mod tokens;

pub struct TokenizeFileQuery;
impl Query for TokenizeFileQuery {
    type From = String;
    type To = Vec<tokens::Token>;
    fn run(db: &Database, src: Self::From) -> Self::To {
        unimplemented!()
    }
}
