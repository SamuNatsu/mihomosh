use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProxyArgs {
    /// Print all proxies
    GetAll,

    /// Update proxy selection
    Update,

    /// Test all proxies
    TestAll,
}
