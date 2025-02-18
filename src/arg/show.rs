use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Show Mihomo current activated profile
    Profile,

    /// Show mihomo current activated rules
    Rules,
}
