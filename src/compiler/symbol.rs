//! # Symbol Interning
//!
//! This module provides a `SymbolFactory` for interning strings. Symbol interning is a
//! technique for storing only one copy of each distinct string. This can save a
//! significant amount of memory, especially in a compiler where the same
//! identifiers and keywords are often repeated.
//!
//! Instead of passing strings around, the compiler can use `Symbol`s, which are
//! lightweight numeric IDs. This makes comparisons, hashing, and storage much
//! more efficient.

use std::collections::HashMap;
use crate::compiler::source_holder::SourceHolder;

use super::span::{SpanWithRef, Span};

/// A rough estimate used to pre-allocate the symbol table's capacity.
/// This assumes an average symbol length to estimate the number of symbols,
/// helping to avoid reallocations of the hash map.
const RECIPROCAL_OF_USUAL_SYMBOL_NUM_PER_LENGTH: usize = 120;

/// Represents a unique identifier for a string.
///
/// A `Symbol` is a lightweight, copyable handle that can be used to refer to a
/// string that has been interned by a `SymbolFactory`. It can be cheaply
/// compared for equality.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Symbol(u32);

/// A factory for creating and managing symbols.
///
/// The `SymbolFactory` is responsible for interning strings from the source
/// code. It maintains a mapping from string slices to `Symbol`s, ensuring that
/// each unique string is associated with a unique `Symbol`.
pub struct SymbolFactory<'src> {
    /// The next available symbol ID.
    now_symbol_id: u32,
    /// A reference to the source code holder.
    source: SourceHolder<'src>,
    /// The interning table, mapping string slices to their corresponding symbols.
    map: HashMap<SpanWithRef<'src>, Symbol>,
}


/// Implementation block for `SymbolFactory`.
///
/// This block contains methods for `SymbolFactory` such as
/// constructor, and methods for interning strings into symbols.
impl <'src>SymbolFactory<'src> {
    /// Creates a new `SymbolFactory` for a given source holder.
    ///
    /// The factory is initialized with an empty symbol table. The capacity of the
    /// table is pre-allocated based on a heuristic to improve performance.
    ///
    /// # Arguments
    ///
    /// * `src` - A reference to the `SourceHolder` containing the source code.
    pub fn new(src: SourceHolder<'src>) -> Self {
        Self {
            map: HashMap::with_capacity(src.len() * 2 / RECIPROCAL_OF_USUAL_SYMBOL_NUM_PER_LENGTH + 1),
            source: src,
            now_symbol_id: 0,
        }
    }

    /// Interns a string slice represented by a `Span`.
    ///
    /// If the string has already been interned, this method returns the existing
    /// `Symbol`. Otherwise, it creates a new `Symbol`, adds it to the symbol table,
    /// and returns it.
    ///
    /// # Arguments
    ///
    /// * `span` - The `Span` representing the string slice to intern.
    ///
    /// # Returns
    ///
    /// The `Symbol` for the given string slice.
    #[inline]
    pub fn from_span(&'src mut self, span: Span) -> Symbol {
        // Create a temporary `SpanWithRef` to perform the lookup.
        let span_with_ref = span.with_ref(&self.source);
        if let Some(&symbol) = self.map.get(&span_with_ref) {
            symbol
        } else {
            // If the symbol is not found, create a new one.
            let symbol_id = self.now_symbol_id;
            let symbol = Symbol(symbol_id);
            self.map.insert(span.with_ref(&self.source), symbol);
            self.now_symbol_id += 1;
            symbol
        }
    }

    /// A convenience method to intern a string from a byte range.
    ///
    /// This method creates a `Span` from the given range and then calls
    /// `from_span`.
    ///
    /// # Arguments
    ///
    /// * `begin` - The starting byte index of the string.
    /// * `end` - The ending byte index of the string.
    ///
    /// # Returns
    ///
    /// The `Symbol` for the given string slice.
    #[inline]
    pub fn from_range(&'src mut self, begin: usize, end: usize) -> Symbol {
        self.from_span(Span::new(begin, end))
    }
}
