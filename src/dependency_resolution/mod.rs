//! This module is responsible for resolving project dependencies.
//! It will handle tasks such as parsing dependency manifests,
//! fetching external crates, and constructing a dependency graph.

mod specify_package_level_dependent;

use crate::compiler::runtime::{Engine, Query};
use crate::parser::ast;
