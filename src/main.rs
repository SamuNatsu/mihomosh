mod cli;

use clap::Parser;

use crate::cli::Cli;

fn main() {
    let _ = Cli::parse();
}
