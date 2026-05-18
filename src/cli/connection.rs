use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConnectionCommand {
    /// List connections
    List {
        /// Filter expression
        #[arg(short, long)]
        filters: Option<String>,
    },

    /// Close connections
    Close {
        /// Filter expression
        #[arg(short, long)]
        filters: Option<String>,
    },
}
