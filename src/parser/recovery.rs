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
        let has_wildcard_first = WhileParsing::FIRST_1_SETS.any;

        // A parser with an unmatched fallback accepts any token as a first
        // token. In that case FIRST cannot be a recovery synchronization set;
        // use FOLLOW (which still contains known declaration boundaries).
        let at_sync_point = if has_wildcard_first {
            is_follow && advanced
        } else {
            (is_first && advanced) || (is_follow && !is_first)
        };
        if at_sync_point {
            break;
        }

        // If parsing failed at its first token, discard that token even when it
        // is also in FOLLOW. Otherwise FIRST/FOLLOW overlap can leave the cursor
        // unchanged and retry the same invalid production forever.
        parser.consume_token();
        advanced = true;
    }

    parser.set_panic_or_backtrack_mode(previous_mode);
}
