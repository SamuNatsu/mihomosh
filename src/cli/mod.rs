pub mod config;
pub mod connection;
pub mod kernel;
pub mod profile;
pub mod proxy;
pub mod rule;

use clap::Parser;
use clap_complete::Shell;

use crate::cli::{
    config::ConfigCommand, connection::ConnectionCommand, kernel::KernelCommand,
    profile::ProfileCommand, proxy::ProxyCommand, rule::RuleCommand,
};

#[derive(Parser)]
#[command(about, version)]
pub enum Cli {
    /// Open TUI
    Tui {
        /// Frame per second for animations
        #[arg(short, long, default_value = "10")]
        fps: u16,
    },

    /// Manage configurations
    #[command(subcommand, visible_alias = "conf")]
    Config(ConfigCommand),

    /// Manage profiles
    #[command(subcommand, visible_alias = "prof")]
    Profile(ProfileCommand),

    /// Manage runtime proxies and proxy groups
    #[command(subcommand)]
    Proxy(ProxyCommand),

    /// Manage runtime rules and rule sets
    #[command(subcommand)]
    Rule(RuleCommand),

    /// Manage runtime connections
    #[command(subcommand, visible_alias = "conn")]
    Connection(ConnectionCommand),

    /// Manage Mihomo kernel
    #[command(subcommand, visible_alias = "meta")]
    Kernel(KernelCommand),

    /// Generate shell completion scripts
    #[command(visible_alias = "comp")]
    Completion {
        /// Shell name
        #[arg(value_enum)]
        shell: Shell,
    },
}
