use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Edit configurations
    Edit {
        /// Editor to be used
        #[arg(long, short)]
        editor: Option<String>,
    },

    /// Reset configurations
    Reset,

    /// View configurations
    View,
}
