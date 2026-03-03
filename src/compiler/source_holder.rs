//! # Source Holder
//!
//! This module provides a `SourceHolder` struct that holds the source code and
//! provides methods for accessing it. It is used by the compiler to keep track
//! of the source code and its line and column information.

use crate::compiler::span::Span;

#[derive(Clone)]
pub struct SourceHolder<'src> {
    /// A reference to the source code string.
    src: &'src str,
    line_starts: Vec<u32>,
}

/// Implementation of `SourceHolder` for managing source code.
///
/// This block provides methods for constructing a `SourceHolder`,
/// accessing the underlying source string, determining its length,
/// and slicing portions of the code based on line and column information.
impl<'src> SourceHolder<'src> {
    pub fn new(source: &'src str, line_starts: Vec<u32>) -> Self {
        Self {
            src: source,
            line_starts,
        }
    }

    pub fn get_snippet(&self, span: Span) -> &'src str {
        &self.src[span.begin as usize..span.end as usize]
    }

    /// エラー表示時などに (行, 列) を計算する
    pub fn resolve_position(&mut self, offset: u32) -> (usize, usize) {
        let line_idx = match self.line_starts.binary_search(&offset) {
            Ok(idx) => idx,
            Err(idx) => idx - 1,
        };
        let column = (offset - self.line_starts[line_idx]) as usize;
        (line_idx + 1, column + 1)
    }

    pub fn len(&self) -> usize {
        self.src.len()
    }

    pub fn is_empty(&self) -> bool {
        self.src.is_empty()
    }

    pub fn get(&self) -> &str {
        self.src
    }
}
