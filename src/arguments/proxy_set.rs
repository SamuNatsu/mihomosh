use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProxySetArgs {
    /// Print all proxy sets
    GetAll,

    /// Update all proxy sets
    UpdateAll,

    /// Test all proxy sets
    TestAll,
}
