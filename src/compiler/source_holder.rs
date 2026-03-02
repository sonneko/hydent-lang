//! # Source Holder
//!
//! This module provides a `SourceHolder` struct that holds the source code and
//! provides methods for accessing it. It is used by the compiler to keep track
//! of the source code and its line and column information.

/// Holds the source code and provides methods for accessing it.
#[derive(Copy, Clone)]
pub struct SourceHolder<'src> {
    /// A reference to the source code string.
    src: &'src str,
}

/// Implementation of `SourceHolder` for managing source code.
///
/// This block provides methods for constructing a `SourceHolder`,
/// accessing the underlying source string, determining its length,
/// and slicing portions of the code based on line and column information.
impl<'src> SourceHolder<'src> {
    /// Creates a new `SourceHolder`.
    ///
    /// This function preprocesses the source code to find the starting
    /// index of each line, which allows for efficient line and column
    /// based lookups later.
    ///
    /// # Arguments
    ///
    /// * `source` - The source code string.
    pub fn new(source: &'src str) -> Self {
        Self { src: source }
    }

    /// Returns a reference to the source code string.
    pub fn get_source_ref(&self) -> &'src str {
        self.src
    }

    /// Returns the total length of the source code in bytes.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.src.len()
    }
}
