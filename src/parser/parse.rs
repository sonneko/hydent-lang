use crate::compiler::arena::ArenaBox;
use crate::compiler::context::frontend::CompilerFrontendContext;
use crate::diagnostic::stream::DiagnosticStream;
use crate::parser::generated_ast::Module;
use crate::parser::generated_parser::GeneratedParser;
use crate::tokenizer::token_stream::TokenStream;

pub struct Parser<'ctx, 'src, 's, S: DiagnosticStream> {
    pub ctx: CompilerFrontendContext<'ctx, 'src>,
    pub tokens: TokenStream,
    pub diagnostic_stream: &'s mut S,
}

impl<'ctx, 'src, 's, S: DiagnosticStream> Parser<'ctx, 'src, 's, S> {
    pub fn new(
        tokens: TokenStream,
        ctx: CompilerFrontendContext<'ctx, 'src>,
        diagnostic_stream: &'s mut S,
    ) -> Parser<'ctx, 'src, 's, S> {
        Self {
            ctx,
            tokens,
            diagnostic_stream,
        }
    }

    pub fn parse(&mut self) -> ArenaBox<Module> {
        match self.parse_Module() {
            Ok(module) => self.ctx.ast_arena.alloc(module),
            Err(err) => {
                self.diagnostic_stream.pour(err);
                self.ctx.ast_arena.alloc(Module {
                    TopLevelStatement: { self.ctx.ast_arena.alloc_with(|| None) },
                })
            }
        }
    }
}
