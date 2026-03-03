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

use crate::compiler::source_holder::SourceHolder;
use std::collections::HashMap;

use crate::compiler::span::Span;

/// A rough estimate used to pre-allocate the symbol table's capacity.
/// This assumes an average symbol length to estimate the number of symbols,
/// helping to avoid reallocations of the hash map.
const RECIPROCAL_OF_USUAL_SYMBOL_NUM_PER_LENGTH: usize = 120;

/// Represents a unique identifier for a string.
///
/// A `Symbol` is a lightweight, copyable handle that can be used to refer to a
/// string that has been interned by a `SymbolFactory`. It can be cheaply
/// compared for equality.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Symbol(u32);

impl Symbol {
    pub fn raw(&self) -> u32 {
        self.0
    }
}

/// A factory for creating and managing symbols.
///
/// The `SymbolFactory` is responsible for interning strings from the source
/// code. It maintains a mapping from string slices to `Symbol`s, ensuring that
/// each unique string is associated with a unique `Symbol`.
pub struct SymbolFactory<'src> {
    /// The next available symbol ID.
    now_symbol_id: u32,
    /// A reference to the source code holder.
    source: &'src str,
    /// The interning table, mapping string slices to their corresponding symbols.
    map: HashMap<&'src str, u32>,
    reverse: Vec<&'src str>,
}

/// Implementation block for `SymbolFactory`.
///
/// This block contains methods for `SymbolFactory` such as
/// constructor, and methods for interning strings into symbols.
impl<'src> SymbolFactory<'src> {
    /// Creates a new `SymbolFactory` for a given source holder.
    ///
    /// The factory is initialized with an empty symbol table. The capacity of the
    /// table is pre-allocated based on a heuristic to improve performance.
    ///
    /// # Arguments
    ///
    /// * `src` - A reference to the `SourceHolder` containing the source code.
    pub fn new(src: &'src str) -> Self {
        Self {
            map: HashMap::with_capacity(
                src.len() * 2 / RECIPROCAL_OF_USUAL_SYMBOL_NUM_PER_LENGTH + 1,
            ),
            reverse: Vec::with_capacity(
                src.len() * 2 / RECIPROCAL_OF_USUAL_SYMBOL_NUM_PER_LENGTH + 1,
            ),
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
    pub fn from_span(&mut self, span: Span) -> Symbol {
        // Create a temporary `SpanWithRef` to perform the lookup.
        let span_with_ref = span.into(self.source);
        if let Some(&id) = self.map.get(span_with_ref) {
            Symbol(id)
        } else {
            // If the symbol is not found, create a new one.
            let symbol_id = self.now_symbol_id;
            let symbol = Symbol(symbol_id);
            self.map.insert(span.into(self.source), symbol.raw());
            self.reverse.push(span.into(self.source));
            self.now_symbol_id += 1;
            symbol
        }
    }

    pub fn from_range(&mut self, begin: usize, end: usize) -> Symbol {
        let span = Span::new(begin, end);
        self.from_span(span)
    }

    pub fn get(&self, symbol: &Symbol) -> &'src str {
        self.reverse.get(symbol.raw() as usize).unwrap()
    }
}
