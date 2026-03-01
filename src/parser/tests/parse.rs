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
    },
    tokenizer::{token_stream::TokenStream, tokenize::Tokenizer},
};

fn parse(source: &str) -> (Module, Arena, Arena) {
    let source = String::from(source);
    let mut symbols = SymbolFactory::new(SourceHolder::new(&source));
    let tokenizer = Tokenizer::new(&source, &mut symbols);
    let (tokens, errors) = tokenizer.tokenize();
    let mut ast_arena = Arena::new();
    let mut errors_arena = Arena::new();
    let ast = {
        let mut parser = Parser::new(
            TokenStream::new(tokens),
            CompilerFrontendContext {
                source: SourceHolder::new(&source),
                symbol_factory: symbols,
                ast_arena: &mut ast_arena,
                errors_arena: &mut errors_arena,
            },
        );
        let ast = parser.parse();
        ast
    };
    (ast.unwrap(), ast_arena, errors_arena)
}

#[test]
fn test_parse() {
    let (mut ast, mut arena, _) = parse("fn main(){} fn d() {}");

    println!("{:?}", ast);
    // Module {
    //     TopeLvelStatement: []
    // }

    // TODO: Gemini! Your task is here.
}
