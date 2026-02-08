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
    pub fn len(&self) -> usize {
        self.src.len()
    }

    pub fn upgrade(self) -> SourceHolderWithLineInfo<'src> {
        let lines = self
            .src
            .match_indices("\n")
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        SourceHolderWithLineInfo {
            src: self.src,
            line_starts: lines,
        }
    }
}

pub struct SourceHolderWithLineInfo<'src> {
    src: &'src str,
    line_starts: Vec<usize>,
}

impl<'src> SourceHolderWithLineInfo<'src> {
    /// Extracts a slice of the source code given 1-based line and 0-based column numbers.
    ///
    /// # Panics
    ///
    /// This function will panic if the line numbers are out of bounds or if the
    /// resulting slice is not valid.
    ///
    /// # Arguments
    ///
    /// * `begin_line` - The starting line number (1-based).
    /// * `begin_column` - The starting column number (0-based from the start of the line).
    /// * `end_line` - The ending line number (1-based).
    /// * `end_column` - The ending column number (0-based from the start of the line).
    ///
    /// # Returns
    ///
    /// A string slice of the specified portion of the source code.
    pub fn slice_from_line_info(
        &self,
        begin_line: usize,
        begin_column: usize,
        end_line: usize,
        end_column: usize,
    ) -> &str {
        let begin = self.line_starts[begin_line - 1] + begin_column;
        let end = self.line_starts[end_line - 1] + end_column;
        &self.src[begin..end]
    }
}
