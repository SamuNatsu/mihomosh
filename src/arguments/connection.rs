use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConnectionArgs {
    /// Print all connections
    View,

    /// Close filtered connections, close all when no filter provided
    Close {
        /// Network type filter, exact match
        #[arg(short, long)]
        r#type: Option<Vec<String>>,

        /// Host filter, support `*` and `?` wildcards
        #[arg(short = 'H', long)]
        host: Option<Vec<String>>,

        /// Process filter, exact match
        #[arg(short, long)]
        process: Option<Vec<String>>,

        /// Source address filter, support `*` and `?` wildcards
        #[arg(short, long)]
        source: Option<Vec<String>>,

        /// Destination address filter, support `*` and `?` wildcards
        #[arg(short, long)]
        destination: Option<Vec<String>>,

        /// Chain filter, exact match
        #[arg(short, long)]
        chain: Option<Vec<String>>,

        /// Rule filter, exact match
        #[arg(short, long)]
        rule: Option<Vec<String>>,
    },
}
