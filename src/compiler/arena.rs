//! # Arena Allocator
//!
//! This module provides an arena allocator for the Hydent compiler. Arena allocation
//! is a memory management technique where memory for a group of objects is
//! allocated from a single large block of memory. This can be more efficient
//! than allocating each object individually, especially for a large number of
//! small objects.
//!
//! The `Arena` struct is the main entry point to the arena allocator. It
//! provides methods for allocating objects and vectors within the arena.

use bumpalo::Bump;
use bumpalo::collections;

/// A memory arena for allocating objects.
pub struct Arena {
    arena: Bump,
}

/// A smart pointer for an object allocated in an `Arena`.
pub struct ArenaBox<'arena, T>(&'arena mut T);

/// A growable list of objects allocated in an `Arena`.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ArenaVec<'arena, T>(collections::Vec<'arena, T>);

/// Implementation of `Arena` for managing memory allocations.
///
/// This block provides methods for constructing an `Arena` and
/// for allocating various data structures such as boxed values and
/// vectors within the arena's managed memory.
impl Arena {
    /// Creates a new `Arena`.
    pub fn new() -> Self {
        Self {
            arena: Bump::new(),
        }
    }

    /// Allocates a new object in the arena.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to allocate.
    ///
    /// # Returns
    ///
    /// An `ArenaBox` pointing to the allocated object.
    pub fn alloc_box<T>(&mut self, value: T) -> ArenaBox<'_, T> {
        ArenaBox(self.arena.alloc(value))
    }

    /// Allocates a new vector in the arena.
    ///
    /// # Arguments
    ///
    /// * `len` - The length of the vector.
    /// * `cap` - The capacity of the vector.
    ///
    /// # Returns
    ///
    /// An `ArenaVec` pointing to the allocated vector.
    pub fn alloc_vec<T>(&mut self, len: usize, cap: usize) -> ArenaVec<'_, T> {
        let vec = collections::Vec::with_capacity_in(cap, &self.arena);
        ArenaVec(vec)
    }
}

impl<'arena, T> ArenaVec<'arena, T> {

    /// Appends a value to the vector.
    /// 
    /// # Arguments
    ///
    /// * `value` - The value to append.
    /// 
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

}

