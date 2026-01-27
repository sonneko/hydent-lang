//! # The Compiler Infrastructure
//!
//! This module provides the core infrastructure for the Hydent compiler. It includes
//! fundamental components like symbol management, source code handling, and memory
//! allocation arenas. This infrastructure is used by the various compiler passes
//! to process and transform the source code.

pub mod symbol;
pub mod span;
pub mod arena;
pub mod context;
pub mod source_holder;
pub mod query_sys;
