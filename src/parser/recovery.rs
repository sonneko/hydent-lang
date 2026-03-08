use crate::{
    diagnostic::stream::DiagnosticStream,
    parser::{ast_node::ASTNode, base_parser::BaseParser, parse::Parser, tracer::Tracer},
};

pub fn recover<'ctx, 'src, 's, WhileParsing, S, TR>(parser: &mut Parser<'ctx, 'src, 's, S, TR>)
where
    WhileParsing: ASTNode,
    S: DiagnosticStream,
    TR: Tracer,
{
    parser.set_panic_or_backtrack_mode(true);
    panic!("Error");
    parser.set_panic_or_backtrack_mode(false);
}
