use crate::compiler::{
    arena::Arena, context::frontend::CompilerFrontendContext, query_sys::Query,
    source_holder::SourceHolder, symbol::SymbolFactory,
};
use crate::diagnostic::CompilerDiagnostic;
use crate::parser::parser::Parser;
use crate::tokenizer::{tokenizer::Tokenizer, tokens::Token};

pub mod ast;
pub mod errors;
pub mod generated_ast;
pub mod generated_parser;
pub mod parser;

pub struct ParseFileQuery;
impl Query for ParseFileQuery {
    type From = String;
    type To = generated_ast::Module;
    fn run(
        db: &crate::compiler::query_sys::Database,
        src: Self::From,
    ) -> Result<Self::To, Box<dyn CompilerDiagnostic>> {
        // let source_holder = SourceHolder::new(&src);
        // let arena = Arena::new();
        // let mut ctx = CompilerFrontendContext {
        //     source: source_holder,
        //     symbol_factory: SymbolFactory::new(source_holder),
        //     arena: &arena,
        // };
        // let tokens = {
        //     let tokenizer = Tokenizer::new(&src, &mut ctx.symbol_factory);
        //     tokenizer.tokenize().map_err(|e| e.into())?
        // };

        // let parser = Parser::new(
        //     tokens.into_iter().peekable(),
        //     ctx,
        // );
        // parser.parse().map_err(|e| e.into())
        unimplemented!()
    }
}
