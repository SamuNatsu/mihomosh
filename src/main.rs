mod arguments;
mod commands;
mod includes;
mod models;
mod utils;

use std::io;

use anyhow::Result;
use clap::{CommandFactory, Parser};

use crate::{
    arguments::Args,
    commands::{config, connection, control, inspect, profile},
};

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        println_danger!("{err:?}");
    }
}

async fn run() -> Result<()> {
    match Args::parse() {
        Args::Config(args) => config::handle_config(args).await?,
        Args::Profile(args) => profile::handle_profile(args).await?,
        Args::Connection(args) => connection::handle_connection(args).await?,
        Args::Inspect(args) => inspect::handle_inspect(args).await?,
        Args::Control(args) => control::handle_control(args).await?,
        Args::Proxy(args) => todo!(),
        Args::ProxySet(args) => todo!(),
        Args::Rule => todo!(),
        Args::RuleSet(args) => todo!(),
        Args::ShellCompletion { shell } => {
            let mut cmd = Args::command();
            let bin_name = cmd.get_name().to_owned();
            clap_complete::generate(shell, &mut cmd, bin_name, &mut io::stdout());
        }
    }
    Ok(())
}
