use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProxyCommand {
    /// List proxies/proxy groups
    ///
    /// By default, proxy groups is listed
    #[command(visible_alias = "ls")]
    List {
        /// List proxies
        #[arg(short, long)]
        proxies: bool,
    },

    /// Select proxy for selectable proxy group
    #[command(visible_alias = "sel")]
    Select,

    /// Test proxies
    Test {
        /// Test URL
        #[arg(short, long, default_value = "https://cp.cloudflare.com/generate_204")]
        url: String,

        /// Test timeout millisecond
        #[arg(short, long, default_value = "5000")]
        timeout: u32,
    },
}
