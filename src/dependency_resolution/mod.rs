//! This module is responsible for resolving project dependencies.
//! It will handle tasks such as parsing dependency manifests,
//! fetching external crates, and constructing a dependency graph.

use crate::compiler::query_sys::Query;
use crate::parser::ast;

pub struct SpecifyPackageLevelDependenciesQuery;
impl Query for SpecifyPackageLevelDependenciesQuery {
    type From = ast::Module;
    type To = ();
    fn run(db: &crate::compiler::query_sys::Database, src: Self::From) -> Self::To {
        // TODO: implement
        todo!()
    }
}
