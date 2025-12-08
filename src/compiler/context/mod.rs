//! # Compiler Context
//!
//! This module defines the `CompilerContext` trait and related utilities, which
//! are used to manage and pass around compiler-wide information. This includes
//! things like the source code, symbol table, and arena allocator.
//!
//! The context is designed to be modular and extensible, allowing different
//! compiler passes to access the information they need without having to pass
//! around a large number of individual arguments.

pub mod frontend;
pub mod middleend;
pub mod backend;

use crate::compiler::source_holder::SourceHolder;

use super::symbol::SymbolFactory;
use super::arena::Arena;

/// A trait for types that can be merged.
///
/// This trait is used to merge compiler contexts from different compiler
/// passes.
pub trait Mergeble: Sized {
    /// Merges this value with another value of the same type.
    /// This is typically used to combine contexts from parallel processing or different compilation stages.
    fn merge(self, other: Self) -> Self;
}

/// Recursively merges a vector of mergeable values.
///
/// This function uses a divide-and-conquer approach to merge the values.
fn recursive_merge<T: Mergeble>(mut parts: Vec<T>) -> T {
    let len = parts.len();
    if len == 1 {
        return parts.remove(0);
    }
    let mid = len / 2;
    let right_parts = parts.split_off(mid);
    let left_parts = parts;
    let left_result = recursive_merge(left_parts);
    let right_result = recursive_merge(right_parts);
    left_result.merge(right_result)
}

/// Integrates all compiler contexts into a single context.
///
/// This function is used to merge the contexts from all compiler passes
/// into a single context that can be used by the backend.
///
/// # Panics
///
/// This function will panic if the `contexts` vector is empty.
pub fn integrate_all_contexts<T: Mergeble>(contexts: Vec<T>) -> T {
    if contexts.is_empty() {
        panic!("contexts is empty");
    }
    recursive_merge(contexts)
}

/// A trait for compiler contexts.
///
/// This trait defines the common interface for all compiler contexts.
/// It provides methods for accessing the source code and for transitioning
/// to the next compiler pass.
pub trait CompilerContext: Sized + Mergeble {
    /// The type of the next compiler pass.
    type NextFase;

    /// Transitions the current compiler context to the next phase of the compilation process.
    /// This method consumes the current context and returns a new context suitable for the next stage.
    fn next_fase(self) -> Self::NextFase;

    /// Returns a reference to the source holder, which contains the original source code
    /// and possibly metadata about it.
    fn get_source(&self) -> &SourceHolder<'_>;

}
