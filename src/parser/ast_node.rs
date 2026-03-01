use crate::parser::errors::ParseErr;
use crate::parser::generated_ast::ASTVisitor;
use crate::tokenizer::tokens::Token;

pub trait ASTNode:
    Copy + Clone + std::fmt::Debug + std::hash::Hash + PartialEq + Eq + 'static + Sized
{
    const SYNC_POINT_SETS: TokenBitMap;
    const FIRST_SETS: TokenBitMap;
    fn get_error_situation(err: ParseErr) -> Option<Self>;

    fn accept<V: ASTVisitor>(&self, visitor: &mut V);

    fn is_follow_sets(token: &Option<Token>) -> bool {
        Self::is_sync_point(token)
    }

    fn is_sync_point(token: &Option<Token>) -> bool {
        Self::SYNC_POINT_SETS.contains(token)
    }

    fn is_first_sets(token: &Option<Token>) -> bool {
        Self::FIRST_SETS.contains(token)
    }
}

pub struct TokenBitMap {
    keywords: u64,
    operators: u64,
    delimiter: u64,
    literals: bool,
    identifier: bool,
    eof: bool,
}

impl TokenBitMap {
    pub const fn build_map(identifier: bool, literals: bool, eof: bool, tokens: &[Token]) -> Self {
        let mut keywords_bits = 0u64;
        let mut operators_bits = 0u64;
        let mut delimiters_bits = 0u64;

        let mut i = 0usize;

        while i < tokens.len() {
            // can't use "for in loop" and iterator pattern in const function
            match tokens[i] {
                Token::Keyword(keyword) => keywords_bits |= 1 << (keyword as u8),
                Token::Operator(operator) => operators_bits |= 1 << (operator as u8),
                Token::Delimiter(delimiter) => delimiters_bits |= 1 << (delimiter as u8),
                _ => panic!("Invalid token in SyncPointBitMap"),
            }
            i += 1;
        }

        Self {
            keywords: keywords_bits,
            operators: operators_bits,
            delimiter: delimiters_bits,
            literals,
            identifier,
            eof,
        }
    }

    pub fn contains(&self, token: &Option<Token>) -> bool {
        if let &Some(token) = token {
            match token {
                Token::Keyword(keyword) => (1 << (keyword as u8)) & self.keywords != 0,
                Token::Operator(operator) => (1 << (operator as u8)) & self.operators != 0,
                Token::Delimiter(delimiter) => (1 << (delimiter as u8)) & self.delimiter != 0,
                Token::Literal(_) => self.literals,
                Token::Identifier(_) => self.identifier,
                Token::EndOfFile => self.eof,
                Token::Comment(_) => false,
                Token::Invalid => false,
            }
        } else {
            self.eof
        }
    }
}
