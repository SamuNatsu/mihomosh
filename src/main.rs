mod args;
mod commands;
mod models;
mod utils;

use std::io;

use anyhow::Result;
use clap::{CommandFactory, Parser};

use crate::{
    args::{Args, config::ConfigArgs, inspect::InspectArgs},
    commands::{config, inspect},
};

#[tokio::main]
async fn main() -> Result<()> {
    match Args::parse() {
        Args::Config(args) => match args {
            ConfigArgs::View { viewer } => config::view(viewer)?,
            ConfigArgs::Edit { editor } => config::edit(editor)?,
            ConfigArgs::Reset => config::reset()?,
        },
        Args::Inspect(args) => match args {
            InspectArgs::Log => inspect::log().await?,
            InspectArgs::Traffic => inspect::traffic().await?,
            InspectArgs::Memory => inspect::memory().await?,
            InspectArgs::Version => inspect::version().await?,
        },
        Args::ShellCompletion { shell } => {
            let mut cmd = Args::command();
            let bin_name = cmd.get_name().to_owned();

            clap_complete::generate(shell, &mut cmd, bin_name, &mut io::stdout());
        }
        _ => (),
    }

    Ok(())
}
