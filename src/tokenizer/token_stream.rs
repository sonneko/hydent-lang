use crate::{compiler::span::Span, tokenizer::tokens::Token, type_checker};

pub struct TokenStream {
    tokens: Vec<(Token, Span)>,
    cursor: usize,
    checkpoints: Vec<usize>,
}

impl TokenStream {
    pub fn new(mut tokens: Vec<(Token, Span)>) -> Self {
        tokens.retain(|(token, _)| {
            !matches!(
                token,
                Token::Comment(crate::tokenizer::tokens::Comment::LineComment) |
                Token::Comment(crate::tokenizer::tokens::Comment::BlockComment)
            )
        });
        Self {
            tokens,
            cursor: 0,
            checkpoints: Vec::with_capacity(10),
        }
    }

    pub fn peek(&self, index: usize) -> Option<(Token, Span)> {
        self.tokens.get(self.cursor + index).copied()
    }

    pub fn checkpoint(&mut self) {
        self.checkpoints.push(self.cursor);
    }

    pub fn rollback(&mut self) {
        self.cursor = self
            .checkpoints
            .pop()
            .expect("No checkpoint to rollback to!");
    }

    pub fn commit(&mut self) {
        self.checkpoints.pop();
    }

    pub fn is_eof(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    pub fn get_now_span(&self) -> Span {
        self.tokens
            .get(self.cursor)
            .map(|(_, span)| *span)
            .unwrap_or_else(|| self.tokens.last().map(|(_, span)| *span).unwrap())
    }
}

impl std::iter::Iterator for TokenStream {
    type Item = (Token, Span);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.tokens.get(self.cursor).copied();
        self.cursor += 1;
        next
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        while let Some(token) = self.peek(i) {
            writeln!(f, "{} in {}", token.0, token.1)?;
            i += 1;
        }
        Ok(())
    }
}
