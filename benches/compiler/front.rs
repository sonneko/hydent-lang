use std::hint::black_box;

use criterion::{criterion_group, Criterion};

use hydent_lang_compiler::{
    compiler::{
        arena::Arena, context::frontend::CompilerFrontendContext, source_holder::SourceHolder,
        symbol::SymbolFactory,
    },
    diagnostic::stream::IgnoreDiagnosticStream,
    parser::{parse::Parser, Ast},
    tokenizer::{token_stream::TokenStream, tokenize::Tokenizer},
};

criterion_group!(front_benches, bench_parser,);

fn bench_parser(c: &mut Criterion) {
    c.bench_function("parse_line_18_program", |b| {
        let source = r#"
pub fn fizz_buzz(n: Int) {
    for i in 1..=n {
        let result = match (i % 3 == 0, i % 5 == 0) {
            (true, true) => "FizzBuzz",
            (true, false) => "Fizz",
            (false, true) => "Buzz",
            _ => "None",
        };
        if result == "None" {
            // 数値そのものを出力
            println(i);
        } else {
            /* FizzまたはBuzzまたはFizzBuzzを出力 */
            println(result);
        }
    }
}
        "#;
        b.iter(black_box(|| {
            let mut diagnostic_stream = IgnoreDiagnosticStream::new();
            let mut symbols = SymbolFactory::new(source);
            let tokenizer = Tokenizer::new(source, &mut symbols);
            let (tokens, line_starts) = tokenizer.tokenize(&mut diagnostic_stream);
            let stream = TokenStream::new(tokens);
            let mut ast_arena = Arena::new();
            let mut errors_arena = Arena::new();
            let mut parser = Parser::new(
                stream,
                CompilerFrontendContext {
                    source,
                    symbol_factory: &mut symbols,
                    ast_arena: &mut ast_arena,
                },
                &mut diagnostic_stream,
            );
            let ast = parser.parse();

            Ast::new(
                ast,
                ast_arena,
                SourceHolder::new(source, line_starts),
                symbols,
            );
        }));
    });
}
