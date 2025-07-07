use clap::Subcommand;

#[derive(Subcommand)]
pub enum ControlArgs {
    /// Flush fake IP cache
    FlushCache,

    /// Update external UI
    UpdateUi,

    /// Update GEO database
    UpdateGeo,

    /// Restart kernal
    Restart,
}
