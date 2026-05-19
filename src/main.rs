mod cli;
mod models;
mod utils;

use clap::Parser;

use crate::cli::Cli;

fn main() {
    let _ = Cli::parse();
}
