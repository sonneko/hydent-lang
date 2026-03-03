use crate::compiler::context::middleend::CompilerMiddleendContext;
use crate::compiler::context::Arena;
use crate::compiler::context::CompilerContext;
use crate::compiler::context::Mergeble;
use crate::compiler::source_holder::SourceHolder;
use crate::compiler::symbol::SymbolFactory;

/// Represents the frontend context of the compiler.
///
/// This struct encapsulates all the necessary data and utilities
/// required during the frontend phase of compilation, such as
/// managing source code, symbols, and AST allocation.
pub struct CompilerFrontendContext<'ctx, 'src> {
    pub source: &'src str,
    pub symbol_factory: &'ctx mut SymbolFactory<'src>,
    pub ast_arena: &'ctx Arena,
    pub errors_arena: &'ctx Arena,
}

/// Implements the `Mergeble` trait for `CompilerFrontendContext`.
///
/// This allows instances of `CompilerFrontendContext` to be merged,
/// which is useful for combining contexts from different compilation units
/// or parallel processing.
impl Mergeble for CompilerFrontendContext<'_, '_> {
    /// Merges this context with another `CompilerFrontendContext`.
    ///
    /// The specific merging logic is currently unimplemented.
    fn merge(self, _other: Self) -> Self {
        // TODO: merge ctx
        unimplemented!()
    }
}

/// Implements the `CompilerContext` trait for `CompilerFrontendContext`.
///
/// This provides the necessary interface for the frontend context to
/// participate in the overall compilation pipeline, defining its next phase
/// and how to access the source holder.
impl<'src> CompilerContext for CompilerFrontendContext<'_, 'src> {
    type NextFase = CompilerMiddleendContext;

    /// Transitions the current frontend context to the next compilation phase, which is the middle-end.
    ///
    /// The specific transition logic is currently unimplemented.
    fn next_fase(self) -> Self::NextFase {
        // TODO
        unimplemented!()
    }
}

/// Implementation of `CompilerFrontendContext` for managing frontend-specific operations.
///
/// This block provides methods for constructing the frontend context,
/// and accessing its internal components like the arena allocator.
impl<'ctx, 'src> CompilerFrontendContext<'ctx, 'src> {
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
    pub fn new(
        source: &'src str,
        ast_arena: &'ctx Arena,
        errors_arena: &'ctx Arena,
        symbol_factory: &'ctx mut SymbolFactory<'src>,
    ) -> CompilerFrontendContext<'ctx, 'src> {
        Self {
            symbol_factory,
            source,
            ast_arena,
            errors_arena,
        }
    }
}
