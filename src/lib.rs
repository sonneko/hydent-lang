#![deny(clippy::all, rust_2018_idioms)]
#![allow(unused)] // should delete this line later

pub mod compiler;
pub mod utility;

// Each module is a pass of the compiler
pub mod cli;
pub mod dependency_resolution;
pub mod diagnostic;
pub mod doc_gen;
pub mod hir_gen;
pub mod hir_transform;
pub mod linker;
pub mod linter;
pub mod llvmir_gen;
pub mod mir_gen;
pub mod mir_transform;
pub mod name_resolution;
pub mod parser;
pub mod tokenizer;
pub mod type_checker;
