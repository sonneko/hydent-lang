//! The main library for the Hydent programming language.
//! This crate contains the core components of the compiler,
//! including the tokenizer, parser, and various compiler passes.

#![deny(clippy::all, rust_2018_idioms)]
#![deny(clippy::empty_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(unused)]

// Each module is a pass of the compiler
mod compiler;
mod utility;

mod cli;
mod dependency_resolution;
mod diagnostic;
mod doc_gen;
mod hir_gen;
mod hir_transform;
mod linker;
mod linter;
mod llvmir_gen;
mod mir_gen;
mod mir_transform;
mod name_resolution;
mod parser;
mod tokenizer;
mod type_checker;
