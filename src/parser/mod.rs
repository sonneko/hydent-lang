use crate::compiler::{
    arena::Arena, context::frontend::CompilerFrontendContext, query_sys::Query,
    source_holder::SourceHolder, symbol::SymbolFactory,
};
use crate::diagnostic::CompilerDiagnostic;
use crate::parser::{errors::ParseErr, parser::Parser};
use crate::tokenizer::{tokenizer::Tokenizer, tokens::Token};

pub mod ast;
pub mod base_parser;
pub mod errors;
pub mod generated_ast;
pub mod generated_parser;
pub mod manual_ast;
pub mod manual_parser;
pub mod parser;

pub struct ParseResult {
    pub ast: generated_ast::Module,
    pub diagnostics: Vec<Box<dyn CompilerDiagnostic>>,
}

pub struct ParseFileQuery;
impl Query for ParseFileQuery {
    type From = String;
    type To = ();
    fn run(
        db: &crate::compiler::query_sys::Database,
        src: Self::From,
    ) -> Result<Self::To, Box<dyn CompilerDiagnostic>> {
        let source_holder = SourceHolder::new(&src);
        let ast_arena = Arena::new();
        let errors_arena = Arena::new();
        let mut ctx = CompilerFrontendContext {
            source: source_holder,
            symbol_factory: SymbolFactory::new(source_holder),
            ast_arena: &ast_arena,
            errors_arena: &ast_arena,
        };

        let tokens = {
            let tokenizer = Tokenizer::new(&src, &mut ctx.symbol_factory);
            tokenizer.tokenize().map_err(|e| e.into())?
        };

        let parser = Parser::new(tokens.into_iter().peekable(), ctx);
        let ast = parser
            .parse()
            .map_err(|e| <ParseErr as Into<Box<dyn CompilerDiagnostic>>>::into(e));
        Ok(())
    }
}
