use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Update Mihomo GEO database
    UpdateGeo,

    /// Update Mihomo group selection
    UpdateGroup,

    /// Restart Mihomo
    Restart,
}
