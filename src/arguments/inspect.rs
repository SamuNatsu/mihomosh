use clap::Subcommand;

#[derive(Subcommand)]
pub enum InspectArgs {
    /// Print real-time logs
    Log,

    /// Print real-time traffic
    Traffic,

    /// Print real-time memory
    Memory,

    /// Print kernal version
    Version,
}
