use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProxyArgs {
    /// Print all proxies
    View,

    /// Update proxy selection
    Update,

    /// Test all proxies
    Test {
        /// URL for testing
        #[arg(short, long, default_value = "https://cp.cloudflare.com/generate_204")]
        url: String,

        /// Testing timeout
        #[arg(short, long, default_value_t = 5000)]
        timeout: u64,
    },
}
