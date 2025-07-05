use clap::Subcommand;

#[derive(Subcommand)]
pub enum ControlArgs {
    /// Flush fake IP cache
    FlushCache,

    /// Update GEO database
    UpdateGeo,

    /// Update external UI
    UpdateUi,

    /// Restart kernal
    Restart,
}
