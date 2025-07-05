mod args;

use std::io;

use anyhow::Result;
use clap::{CommandFactory, Parser};

use crate::args::Args;

fn main() -> Result<()> {
    match Args::parse() {
        Args::ShellCompletion { shell } => {
            let mut cmd = Args::command();
            let bin_name = cmd.get_name().to_owned();

            clap_complete::generate(shell, &mut cmd, bin_name, &mut io::stdout());
        }
        _ => (),
    }

    Ok(())
}
