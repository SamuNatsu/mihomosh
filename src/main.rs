mod args;
mod commands;
mod models;
mod utils;

use std::io;

use anyhow::Result;
use clap::{CommandFactory, Parser};

use crate::{
    args::{
        Args, config::ConfigArgs, connection::ConnectionArgs, control::ControlArgs,
        inspect::InspectArgs,
    },
    commands::{config, connection, control, inspect},
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
        Args::Control(args) => match args {
            ControlArgs::FlushCache => control::flush_cache().await?,
            ControlArgs::UpdateUi => control::update_ui().await?,
            ControlArgs::UpdateGeo => control::update_geo().await?,
            ControlArgs::Restart => control::restart().await?,
        },
        Args::Connection(args) => match args {
            ConnectionArgs::View => connection::view().await?,
            ConnectionArgs::Close {
                r#type,
                host,
                process,
                source,
                destination,
                chain,
                rule,
            } => connection::close(r#type, host, process, source, destination, chain, rule).await?,
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
