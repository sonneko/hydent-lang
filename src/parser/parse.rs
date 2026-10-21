use std::marker::PhantomData;

use crate::compiler::arena::ArenaBox;
use crate::compiler::context::frontend::CompilerFrontendContext;
use crate::diagnostic::stream::DiagnosticStream;
use crate::parser::base_parser::{BaseParser, Enviroment};
use crate::parser::errors::ParseErr;
use crate::parser::generated_ast::Module;
use crate::parser::generated_parser::GeneratedParser;
use crate::parser::tracer::Tracer;
use crate::tokenizer::token_stream::TokenStream;
use crate::tokenizer::tokens::Token;

pub struct Parser<'ctx, 'src, 's, S: DiagnosticStream, TR: Tracer> {
    pub ctx: CompilerFrontendContext<'ctx, 'src>,
    pub tokens: TokenStream,
    pub diagnostic_stream: &'s mut S,
    pub(super) deferred_diagnostics: Vec<(ParseErr, Enviroment)>,
    _marker: PhantomData<TR>,
    pub is_panic_or_backtrack_mode: bool,
}

impl<'ctx, 'src, 's, S: DiagnosticStream, TR: Tracer> Parser<'ctx, 'src, 's, S, TR> {
    pub fn new(
        tokens: TokenStream,
        ctx: CompilerFrontendContext<'ctx, 'src>,
        diagnostic_stream: &'s mut S,
    ) -> Parser<'ctx, 'src, 's, S, TR> {
        Self {
            ctx,
            tokens,
            diagnostic_stream,
            deferred_diagnostics: Vec::new(),
            _marker: PhantomData,
            is_panic_or_backtrack_mode: false,
        }
    }

    pub fn parse(&mut self) -> ArenaBox<Module> {
        let parsed = self.parse_Module().and_then(|module| {
            self.expect(&Token::EndOfFile)?;
            Ok(module)
        });

        match parsed {
            Ok(module) => self.ctx.ast_arena.alloc(module),
            Err(err) => {
                self.diagnostic_stream.pour(err, &self.enviroment());
                self.ctx.ast_arena.alloc(Module {
                    declarations: self.ctx.ast_arena.alloc_with(|| None),
                })
            }
        }
    }
}
