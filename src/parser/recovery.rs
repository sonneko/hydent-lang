use crate::{
    compiler::span::Span,
    diagnostic::stream::DiagnosticStream,
    parser::{ast_node::ASTNode, base_parser::BaseParser, parse::Parser, tracer::Tracer},
    tokenizer::tokens::Token,
};

pub fn recover<'ctx, 'src, 's, WhileParsing, S, TR>(
    parser: &mut Parser<'ctx, 'src, 's, S, TR>,
    start_span: Span,
) where
    WhileParsing: ASTNode,
    S: DiagnosticStream,
    TR: Tracer,
{
    let previous_mode = parser.is_panic_or_backtrack_mode();
    parser.set_panic_or_backtrack_mode(true);

    let mut advanced = parser.now_span() != start_span;
    loop {
        let token = parser.peek::<0>();
        if token.is_none() || token == Some(Token::EndOfFile) {
            break;
        }

        let is_first = WhileParsing::is_first1_sets(&token);
        let is_follow = WhileParsing::is_follow_sets(&token);
        if is_first || is_follow {
            if advanced || is_follow {
                break;
            }

            // A parser failed without consuming its starting token. Discard it
            // once so the enclosing repeat loop cannot retry forever.
            parser.consume_token();
            advanced = true;
        } else {
            parser.consume_token();
            advanced = true;
        }
    }

    parser.set_panic_or_backtrack_mode(previous_mode);
}
