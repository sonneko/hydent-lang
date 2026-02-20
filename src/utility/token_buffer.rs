use crate::{compiler::span::Span, tokenizer::tokens::Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Direct,
    Recording,
    Replay { cursor: usize },
}

pub struct TokenBuffer<I>
where
    I: Iterator<Item = (Token, Span)>,
{
    iter: I,
    mode: Mode,
    buffer: Vec<(Token, Span)>,
    checkpoints: Vec<usize>,
}

impl<I> TokenBuffer<I>
where
    I: Iterator<Item = (Token, Span)>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            mode: Mode::Direct,
            buffer: Vec::with_capacity(64),
            checkpoints: Vec::new(),
        }
    }

    pub fn checkpoint(&mut self) {
        let current_pos = match self.mode {
            Mode::Direct => {
                self.mode = Mode::Recording;
                0
            }
            Mode::Recording => self.buffer.len(),
            Mode::Replay { cursor } => cursor,
        };
        self.checkpoints.push(current_pos);
    }

    pub fn rollback(&mut self) {
        if let Some(start_pos) = self.checkpoints.pop() {
            self.mode = Mode::Replay { cursor: start_pos };
        }
    }

    pub fn commit(&mut self) {
        self.checkpoints.pop();
        if self.checkpoints.is_empty() {
            self.mode = Mode::Direct;
        }
    }
}

impl<I> Iterator for TokenBuffer<I>
where
    I: Iterator<Item = (Token, Span)>,
{
    type Item = (Token, Span);

    fn next(&mut self) -> Option<Self::Item> {
        match self.mode {
            Mode::Direct => self.iter.next(),
            Mode::Recording => {
                if let Some(item) = self.iter.next() {
                    self.buffer.push(item);
                    Some(item)
                } else {
                    None
                }
            }
            Mode::Replay { ref mut cursor } => {
                if *cursor < self.buffer.len() {
                    let item = self.buffer[*cursor];
                    *cursor += 1;
                    Some(item)
                } else {
                    if self.checkpoints.is_empty() {
                        self.mode = Mode::Direct;
                        self.buffer.clear();
                    } else {
                        self.mode = Mode::Recording;
                    }
                    self.next()
                }
            }
        }
    }
}
