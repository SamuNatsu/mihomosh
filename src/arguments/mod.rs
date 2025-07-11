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
    /// Manage configs
    #[command(subcommand)]
    Config(ConfigArgs),

    /// Manage profiles
    #[command(subcommand)]
    Profile(ProfileArgs),

    /// Manage connections
    #[command(subcommand)]
    Connection(ConnectionArgs),

    /// Inspect Mihomo runtime info
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

    /// Generate shell completion
    ShellCompletion {
        /// Target shell name
        #[arg(value_enum)]
        shell: Shell,
    },
}
