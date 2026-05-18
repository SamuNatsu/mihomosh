use clap::Subcommand;

#[derive(Subcommand)]
pub enum KernelCommand {
    /// Monitor logs
    Log,

    /// Monitor traffic
    Traffic,

    /// Monitor memory usage
    Memory,

    /// Get kernel version
    Version,

    /// Flush fake IP cache
    FlushCache,

    /// Update UI
    UpdateUi,

    /// Update Geo database
    UpdateGeo,

    /// Restart kernel
    Restart,
}
