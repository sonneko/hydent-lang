use super::Arena;
use super::SourceHolder;
use super::SymbolFactory;
use crate::compiler::context::CompilerContext;
use super::middleend::CompilerMiddleendContext;
use crate::compiler::context::Mergeble;

/// Represents the frontend context of the compiler.
///
/// This struct encapsulates all the necessary data and utilities
/// required during the frontend phase of compilation, such as
/// managing source code, symbols, and AST allocation.
pub struct CompilerFrontendContext<'ctx> {
    pub source: SourceHolder<'ctx>,
    pub symbol_factory: SymbolFactory<'ctx>,
    pub arena: &'ctx Arena,
}

/// Implements the `Mergeble` trait for `CompilerFrontendContext`.
///
/// This allows instances of `CompilerFrontendContext` to be merged,
/// which is useful for combining contexts from different compilation units
/// or parallel processing.
impl Mergeble for CompilerFrontendContext<'_> {
    /// Merges this context with another `CompilerFrontendContext`.
    ///
    /// The specific merging logic is currently unimplemented.
    fn merge(self, _other: Self) -> Self {
        unimplemented!()
    }
}

/// Implements the `CompilerContext` trait for `CompilerFrontendContext`.
///
/// This provides the necessary interface for the frontend context to
/// participate in the overall compilation pipeline, defining its next phase
/// and how to access the source holder.
impl CompilerContext for CompilerFrontendContext<'_> {
    type NextFase = CompilerMiddleendContext;

    /// Transitions the current frontend context to the next compilation phase, which is the middle-end.
    ///
    /// The specific transition logic is currently unimplemented.
    fn next_fase(self) -> Self::NextFase {
        unimplemented!()
    }

    /// Returns a reference to the `SourceHolder` managed by this context.
    ///
    /// This provides access to the source code and related information.
    fn get_source(&self) -> &SourceHolder<'_> {
        &self.source
    }

}

/// Implementation of `CompilerFrontendContext` for managing frontend-specific operations.
///
/// This block provides methods for constructing the frontend context,
/// and accessing its internal components like the arena allocator.
impl<'ctx> CompilerFrontendContext<'ctx> {
    /// Creates a new `CompilerFrontendContext` instance.
    ///
    /// Initializes the context with a new `SourceHolder`, `SymbolFactory`,
    /// and `Arena` for AST nodes.
    ///
    /// # Arguments
    ///
    /// * `source` - The source code as a string slice.
    ///
    /// # Returns
    ///
    /// A new `CompilerFrontendContext` instance.
    pub fn new(source: &'ctx str, arena: &'ctx Arena) -> CompilerFrontendContext<'ctx> {
        let source_holder = SourceHolder::new(source);
        Self {
            source: source_holder.clone(),
            symbol_factory: SymbolFactory::new(source_holder),
            arena: arena,
        }
    }

}
