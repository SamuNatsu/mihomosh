use clap::Subcommand;

#[derive(Subcommand)]
pub enum NetworkArgs {
    /// Print System Network Configuration
    View,

    /// Set System Proxy Environment Variables
    Proxy,

    /// Clear System Proxy Environment Variables
    UnProxy,

/* 待开发
    /// Enable Tun Mode 
    Tun,

    /// Disable Tun Mode
    UnTun,
*/
}
