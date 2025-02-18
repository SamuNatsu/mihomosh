use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Get Mihomo running configurations
    Configs,

    /// Get Mihomo connections
    Connections,

    /// Get Mihomo groups
    Groups,

    /// Get Mihomo version
    Version,
}
