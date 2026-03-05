//! This module contains the command-line interface (CLI) logic for the Hydent compiler.
//! It handles parsing command-line arguments and orchestrating the compilation process
//! based on user input.

use clap::{Parser, Subcommand, ValueEnum};

use crate::parser::parse_for_integration_test;

pub fn call_cli() {
    let parsed = Cli::parse();

    let log = |msg: &str| {
        if parsed.verbose {
            println!("{}", msg);
        }
    };

    match parsed.command {
        Commands::Build { path, emit, out } => {
            println!("Building...");
            match emit {
                EmitItems::Ast => {
                    log("Getting file contents");
                    let source = std::fs::read_to_string(&path).unwrap();
                    log("parsing...");
                    let ast = format!("{}", parse_for_integration_test(&source));
                    log("writing...");
                    std::fs::write(&out, ast).unwrap();
                }
                EmitItems::Hir => {
                    unimplemented!()
                }
                EmitItems::Mir => {
                    unimplemented!()
                }
                EmitItems::Llvmir => {
                    unimplemented!()
                }
                EmitItems::Bin => {
                    unimplemented!()
                }
            }
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(long, global = true, default_value_t = false)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        path: std::path::PathBuf,

        #[arg(long, value_enum, default_value_t = EmitItems::Bin)]
        emit: EmitItems,
        #[arg(long, short)]
        out: std::path::PathBuf,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum EmitItems {
    Ast,
    Hir,
    Mir,
    Llvmir,
    Bin,
}
