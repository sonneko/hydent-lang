use crate::compiler::arena::ArenaBox;
use crate::parser::errors::ParseErr;
use crate::parser::generated_ast::ASTVisitor;
use crate::tokenizer::tokens::Token;

pub trait ASTNode:
    Copy + Clone + std::fmt::Debug + std::hash::Hash + PartialEq + Eq + 'static + Sized
{
    type Target: Copy + Clone + std::fmt::Debug + std::hash::Hash + PartialEq + Eq + 'static + Sized;
    const FOLLOW_SETS: TokenSet;
    const FIRST_1_SETS: TokenSet;
    const FIRST_2_SETS: TokenSet;

    fn get_error_situation(err: ParseErr) -> Option<Self::Target>;

    fn is_follow_sets(token: &Option<Token>) -> bool {
        Self::FOLLOW_SETS.contains(token)
    }

    fn is_sync_point(token: &Option<Token>) -> bool {
        Self::is_first1_sets(token) || Self::is_follow_sets(token)
    }

    fn is_first1_sets(token: &Option<Token>) -> bool {
        Self::FIRST_1_SETS.contains(token)
    }

    fn is_first2_sets(token: &Option<Token>) -> bool {
        Self::FIRST_2_SETS.contains(token)
    }

    fn ast_name() -> &'static str;
}

pub trait Node {
    fn accept<V: ASTVisitor>(&self, visitor: &mut V) -> V::ReturnType;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TokenSet {
    pub keywords: u64,
    pub operators: u64,
    pub delimiter: u64,
    pub literals: bool,
    pub identifier: bool,
    pub comments: bool,
    pub eof: bool,
    /// Matches every non-EOF token. Used by parser-generator fallback rules.
    pub any: bool,
}

impl TokenSet {
    pub const fn empty() -> Self {
        Self {
            keywords: 0,
            operators: 0,
            delimiter: 0,
            literals: false,
            identifier: false,
            comments: false,
            eof: false,
            any: false,
        }
    }

    pub const fn build_map(
        identifier: bool,
        literals: bool,
        comments: bool,
        eof: bool,
        tokens: &[Token],
    ) -> Self {
        let mut keywords_bits = 0u64;
        let mut operators_bits = 0u64;
        let mut delimiters_bits = 0u64;
        let mut any = false;

        let mut i = 0usize;

        while i < tokens.len() {
            match tokens[i] {
                Token::Keyword(keyword) => keywords_bits |= 1 << (keyword as u8),
                Token::Operator(operator) => operators_bits |= 1 << (operator as u8),
                Token::Delimiter(delimiter) => delimiters_bits |= 1 << (delimiter as u8),
                Token::Comment(_) => {}
                Token::Invalid => any = true,
                _ => panic!("Invalid token in TokenSet"),
            }
            i += 1;
        }

        Self {
            keywords: keywords_bits,
            operators: operators_bits,
            delimiter: delimiters_bits,
            literals,
            identifier,
            comments,
            eof,
            any,
        }
    }

    pub const fn union(self, other: Self) -> Self {
        Self {
            keywords: self.keywords | other.keywords,
            operators: self.operators | other.operators,
            delimiter: self.delimiter | other.delimiter,
            literals: self.literals | other.literals,
            identifier: self.identifier | other.identifier,
            comments: self.comments | other.comments,
            eof: self.eof | other.eof,
            any: self.any | other.any,
        }
    }

    pub const fn intersection(self, other: Self) -> Self {
        Self {
            keywords: (self.keywords & other.keywords)
                | if self.any { other.keywords } else { 0 }
                | if other.any { self.keywords } else { 0 },
            operators: (self.operators & other.operators)
                | if self.any { other.operators } else { 0 }
                | if other.any { self.operators } else { 0 },
            delimiter: (self.delimiter & other.delimiter)
                | if self.any { other.delimiter } else { 0 }
                | if other.any { self.delimiter } else { 0 },
            literals: (self.literals & other.literals)
                | (self.any & other.literals)
                | (other.any & self.literals),
            identifier: (self.identifier & other.identifier)
                | (self.any & other.identifier)
                | (other.any & self.identifier),
            comments: (self.comments & other.comments)
                | (self.any & other.comments)
                | (other.any & self.comments),
            eof: self.eof & other.eof,
            any: self.any & other.any,
        }
    }

    pub fn contains(&self, token: &Option<Token>) -> bool {
        if let Some(token) = token {
            if matches!(token, Token::EndOfFile) {
                return self.eof;
            }
            if self.any {
                return true;
            }
            match token {
                Token::Keyword(keyword) => (1 << (*keyword as u8)) & self.keywords != 0,
                Token::Operator(operator) => (1 << (*operator as u8)) & self.operators != 0,
                Token::Delimiter(delimiter) => (1 << (*delimiter as u8)) & self.delimiter != 0,
                Token::Literal(_) => self.literals,
                Token::Identifier(_) => self.identifier,
                Token::Comment(_) => self.comments,
                Token::EndOfFile => self.eof,
                Token::Invalid => false,
            }
        } else {
            self.eof
        }
    }
}

impl<N> ASTNode for ArenaBox<N>
where
    N: ASTNode<Target = N>,
{
    const FIRST_1_SETS: TokenSet = N::FIRST_1_SETS;
    const FIRST_2_SETS: TokenSet = N::FIRST_2_SETS;
    const FOLLOW_SETS: TokenSet = N::FOLLOW_SETS;
    fn ast_name() -> &'static str {
        N::ast_name()
    }

    fn get_error_situation(err: ParseErr) -> Option<Self::Target> {
        N::get_error_situation(err)
    }
    type Target = N;
}

#[cfg(test)]
mod tests {
    use super::TokenSet;
    use crate::tokenizer::tokens::{Keyword, Operator, Token};

    #[test]
    fn invalid_token_marker_matches_every_non_eof_token() {
        let any = TokenSet::build_map(false, false, false, false, &[Token::Invalid]);

        assert!(any.contains(&Some(Token::Keyword(Keyword::Fn))));
        assert!(any.contains(&Some(Token::Operator(Operator::Add))));
        assert!(any.contains(&Some(Token::Invalid)));
        assert!(!any.contains(&Some(Token::EndOfFile)));
        assert!(!any.contains(&None));

        let only_fn =
            TokenSet::build_map(false, false, false, false, &[Token::Keyword(Keyword::Fn)]);
        let intersection = any.intersection(only_fn);
        assert!(intersection.contains(&Some(Token::Keyword(Keyword::Fn))));
        assert!(!intersection.contains(&Some(Token::Keyword(Keyword::Module))));
    }
}
