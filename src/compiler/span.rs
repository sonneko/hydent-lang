//! # Source Code Spans
//!
//! This module provides a `Span` struct that represents a region of source code.
//! Spans are used by the compiler to associate tokens, AST nodes, and other
//! compiler artifacts with their original location in the source code. This is
//! useful for error reporting and other source code analysis tools.

use core::cmp::{Eq, PartialEq};
use core::hash::Hash;

/// Represents a region of source code.
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct Span {
    /// The starting byte index of the span.
    pub begin: u32,
    /// The ending byte index of the span.
    pub end: u32,
}

/// Implementation of `Span` for creating and manipulating source code regions.
///
/// This block provides methods for constructing `Span` instances and
/// for creating `SpanWithRef` instances that include a reference to the
/// actual source code segment.
impl Span {
    /// Creates a new `Span`.
    pub fn new(begin: usize, end: usize) -> Self {
        Self {
            begin: begin as u32,
            end: end as u32,
        }
    }

    /// Creates a `SpanWithRef` from this `Span`.
    ///
    /// A `SpanWithRef` is a `Span` that also holds a reference to the
    /// source code it refers to. This is useful for situations where you
    /// need to access the source code of a span without having to pass
    /// around a `SourceHolder` separately.
    pub fn into(self, src: &str) -> &str {
        let bytes = &src.as_bytes()[self.begin as usize..self.end as usize];
        let reference = std::str::from_utf8(bytes).expect("Invalid UTF-8");
        reference
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.begin, self.end)
    }
}
