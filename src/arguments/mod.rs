pub mod config;
pub mod connection;
pub mod control;
pub mod inspect;
pub mod profile;
pub mod proxy;
pub mod proxy_set;
pub mod rule_set;

use clap::Parser;
use clap_complete::Shell;

use crate::arguments::{
    config::ConfigArgs, connection::ConnectionArgs, control::ControlArgs, inspect::InspectArgs,
    profile::ProfileArgs, proxy::ProxyArgs, proxy_set::ProxySetArgs, rule_set::RuleSetArgs,
};

/// A Command Line Interface for Mihomo
#[derive(Parser)]
#[command(about, version, long_about = None)]
pub enum Args {
    /// Print/Update configs
    #[command(subcommand)]
    Config(ConfigArgs),

    /// Print/Create/Update/Delete profiles
    #[command(subcommand)]
    Profile(ProfileArgs),

    /// Inspect Mihomo info
    #[command(subcommand)]
    Inspect(InspectArgs),

    /// Control Mihomo
    #[command(subcommand)]
    Control(ControlArgs),

    /// Print/Update/Test proxies
    #[command(subcommand)]
    Proxy(ProxyArgs),

    /// Print/Update/Test proxy sets
    #[command(subcommand)]
    ProxySet(ProxySetArgs),

    /// Print rules
    Rule,

    /// Print/Update rule sets
    #[command(subcommand)]
    RuleSet(RuleSetArgs),

    /// Print/Close connections
    #[command(subcommand)]
    Connection(ConnectionArgs),

    /// Generate shell completion
    ShellCompletion {
        /// Target shell name
        #[arg(value_enum)]
        shell: Shell,
    },
}
