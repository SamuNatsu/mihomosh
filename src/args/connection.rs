use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConnectionArgs {
    /// Print all connections
    GetAll,

    /// Close all connections
    CloseAll,
}
