//! # Source Code Spans
//!
//! This module provides a `Span` struct that represents a region of source code.
//! Spans are used by the compiler to associate tokens, AST nodes, and other
//! compiler artifacts with their original location in the source code. This is
//! useful for error reporting and other source code analysis tools.

use core::cmp::{Eq, PartialEq};
use core::hash::Hash;

use crate::compiler::source_holder::SourceHolder;

/// Represents a region of source code.
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct Span {
    /// The starting byte index of the span.
    begin: usize,
    /// The ending byte index of the span.
    end: usize,
}

/// Implementation of `Span` for creating and manipulating source code regions.
///
/// This block provides methods for constructing `Span` instances and
/// for creating `SpanWithRef` instances that include a reference to the
/// actual source code segment.
impl Span {
    /// Creates a new `Span`.
    pub fn new(begin: usize, end: usize) -> Self {
        Self { begin, end }
    }

    /// Creates a `SpanWithRef` from this `Span`.
    ///
    /// A `SpanWithRef` is a `Span` that also holds a reference to the
    /// source code it refers to. This is useful for situations where you
    /// need to access the source code of a span without having to pass
    /// around a `SourceHolder` separately.
    pub fn with_ref<'src>(self, src: SourceHolder<'src>) -> SpanWithRef<'src> {
        SpanWithRef {
            span: self,
            reference: &src.get_source_ref()[self.begin..self.end],
        }
    }
}

/// A `Span` with a reference to the source code it refers to.
pub struct SpanWithRef<'src> {
    /// The underlying span.
    span: Span,
    /// A reference to the source code of the span.
    reference: &'src str,
}

/// Implementation of `Hash` for `SpanWithRef` to allow hashing based on the referenced string.
impl<'src> Hash for SpanWithRef<'src> {
    /// Hashes the `SpanWithRef` instance using the hash of its `reference` field.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.reference.hash(state);
    }
}

/// Implementation of `PartialEq` for `SpanWithRef` to allow comparison based on the referenced string.
impl<'src> PartialEq for SpanWithRef<'src> {
    /// Compares two `SpanWithRef` instances for equality based on their referenced strings.
    fn eq(&self, other: &Self) -> bool {
        self.reference == other.reference
    }
}

/// Implementation of `Eq` for `SpanWithRef`.
///
/// This trait can be implemented automatically for types that
/// implement `PartialEq` and for which `eq` implies `hash` equality.
impl<'src> Eq for SpanWithRef<'src> {}
