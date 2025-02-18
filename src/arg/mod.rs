pub mod config;
pub mod ctrl;
pub mod profile;
pub mod show;
pub mod status;

use clap::Parser;

/// A CLI tool for Mihomo
#[derive(Parser)]
#[command(about, version, long_about = None)]
#[command(propagate_version = true)]
pub enum Args {
    /// Mihomosh configuration subcommand
    #[command(subcommand)]
    Config(config::Command),

    /// Mihomo controlling subcommand
    #[command(subcommand)]
    Ctrl(ctrl::Command),

    /// Mihomo profiles managing subcommand
    #[command(subcommand)]
    Profile(profile::Command),

    /// Mihomo activated profile displaying subcommand
    #[command(subcommand)]
    Show(show::Command),

    /// Mihomo status displaying subcommand
    #[command(subcommand)]
    Status(status::Command),

    /// Mihomo URL testing subcommand
    Test {
        /// URL used to test the latency
        url: String,
    },
}
