use std::collections::HashMap;
use super::span::{SpanWithRef, Span};

const RECIPROCAL_OF_USUAL_SYMBOL_NUM_PER_LENGTH: usize = 120;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Symbol(u32);

pub struct SymbolFactory<'a> {
    now_symbol_id: u32,
    source: &'a str,
    map: HashMap<SpanWithRef<'a>, Symbol>,
}

impl <'a>SymbolFactory<'a> {
    pub fn new(source: &str) -> Self {
        Self {
            source,
            map: HashMap::with_capacity(source.len() * 2 / RECIPROCAL_OF_USUAL_SYMBOL_NUM_PER_LENGTH + 1),
            now_symbol_id: 0,
        }
    }

    #[inline]
    pub fn from_span(&mut self, span: Span) -> Symbol{
        if let Some(&symbol) = self.map.get(&span) {
            symbol
        } else {
            let symbol_id = self.now_symbol_id;
            self.map.insert(span.with_ref(self.source), Symbol(symbol_id));
            self.now_symbol_id += 1;
            Symbol(symbol_id)
        }
    }

    #[inline]
    pub fn from_range(&mut self, begin: usize, end: usize) -> Symbol {
        self.from_span(Span::new(begin, end))
    }
}
