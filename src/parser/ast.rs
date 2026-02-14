use crate::parser::errors::ParseErr;
use crate::tokenizer::tokens::Token;

pub struct SyncPointBitMap {
    keywords: u64,
    operators: u64,
    delimiter: u64,
    literals: bool,
    comments: bool,
    identifier: bool,
    eof: bool,
}

impl SyncPointBitMap {
    pub const fn build_map(tokens: &[Token], comment: bool, identifier: bool, eof: bool) -> Self {
        let mut keywords_bits = 0u64;
        let mut operators_bits = 0u64;
        let mut delimiters_bits = 0u64;

        let i = 0usize;

        while i < tokens.len() {
            // can't use "for in loop" and iterator pattern in const function
            match tokens[i] {
                Token::Keyword(keyword) => keywords_bits |= 1 << (keyword as u8),
                Token::Operator(operator) => operators_bits |= 1 << (operator as u8),
                Token::Delimiter(delimiter) => delimiters_bits |= 1 << (delimiter as u8),
                _ => panic!("Invalid token in SyncPointBitMap"),
            }
        }

        Self {
            keywords: keywords_bits,
            operators: operators_bits,
            delimiter: delimiters_bits,
            literals: true,
            comments: comment,
            identifier: identifier,
            eof: eof,
        }
    }
}

pub trait ASTNode:
    Copy + Clone + std::fmt::Debug + std::hash::Hash + PartialEq + Eq + 'static + Sized
{
    const SYNC_POINT_SETS: SyncPointBitMap;
    fn get_error_situation(err: ParseErr) -> Option<Self>;

    fn is_sync_point(token: Option<&Token>) -> bool {
        let set = Self::SYNC_POINT_SETS;
        if let Some(&token) = token {
            match token {
                Token::Keyword(keyword) => (1 << (keyword as u8)) & set.keywords != 0,
                Token::Operator(operator) => (1 << (operator as u8)) & set.operators != 0,
                Token::Delimiter(delimiter) => (1 << (delimiter as u8)) & set.delimiter != 0,
                Token::Literal(_) => set.literals,
                Token::Identifier(_) => set.identifier,
                Token::EndOfFile => set.eof,
                Token::Comment(_) => set.comments,
            }
        } else {
            false
        }
    }
}
