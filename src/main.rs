mod cli;
mod handlers;
mod models;
mod templates;
mod utils;

use std::io;

use clap::{CommandFactory, Parser};
use eyre::Result;

use crate::{
    cli::Cli,
    handlers::{config, profile},
};

fn main() -> Result<()> {
    // Install panic hook
    color_eyre::install()?;

    // Handle commands
    match Cli::parse() {
        Cli::Tui { .. } => todo!(),
        Cli::Config(cmd) => config::handle(cmd),
        Cli::Profile(cmd) => profile::handle(cmd),
        Cli::Proxy(_) => todo!(),
        Cli::Rule(_) => todo!(),
        Cli::Connection(_) => todo!(),
        Cli::Kernel(_) => todo!(),
        Cli::Completion { shell } => {
            let mut cmd = Cli::command();
            let bin_name = cmd.get_name().to_owned();

            clap_complete::generate(shell, &mut cmd, bin_name, &mut io::stdout());
            Ok(())
        }
    }
}
