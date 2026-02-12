//! This module contains the command-line interface (CLI) logic for the Hydent compiler.
//! It handles parsing command-line arguments and orchestrating the compilation process
//! based on user input.

use clap::Parser;

pub fn call_cli() {
    Cli::parse();
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {}
