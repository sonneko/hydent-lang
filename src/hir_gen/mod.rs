//! This module is responsible for High-Level Intermediate Representation (HIR) generation.
//!
//! It transforms the Abstract Syntax Tree (AST) into a more semantically rich
//! and compiler-friendly representation, performing initial semantic checks
//! and desugaring of high-level language constructs.

mod gen;
mod hir;

use crate::{compiler::runtime::Query, parser::ast};

pub struct HirGenerateQuery;
impl Query for HirGenerateQuery {
    type From = ast::Module;
    type To = ();
    fn run<E: crate::compiler::runtime::Engine>(engine: &E, src: Self::From) -> Self::To {
        ()
    }
}
