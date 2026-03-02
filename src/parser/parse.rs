use crate::compiler::arena::{Arena, ArenaBox, ArenaIter};
use crate::compiler::context::frontend::CompilerFrontendContext;
use crate::parser::base_parser::BaseParser;
use crate::parser::errors::{IParseErr, ParseErr};
use crate::parser::generated_ast::Module;
use crate::parser::generated_parser::GeneratedParser;
use crate::tokenizer::token_stream;
use crate::tokenizer::{token_stream::TokenStream, tokens::Token};

pub struct Parser<'ctx> {
    pub ctx: CompilerFrontendContext<'ctx>,
    pub tokens: TokenStream,
}

impl<'ctx> Parser<'ctx> {
    pub fn new(tokens: TokenStream, ctx: CompilerFrontendContext<'ctx>) -> Parser<'ctx> {
        Self { ctx, tokens }
    }

    pub fn parse(mut self) -> Result<ArenaBox<Module>, ParseErr> {
        match self.parse_Module() {
            Ok(module) => Ok(self.ctx.ast_arena.alloc(module)),
            Err(err) => Err(err),
        }
    }
}
