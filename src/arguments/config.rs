use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigArgs {
    /// View configs
    View {
        /// Viewer to be used for viewing
        #[arg(short, long, default_value = "less")]
        viewer: String,
    },

    /// Edit configs
    Edit {
        /// Editor to be used for editing
        #[arg(short, long)]
        editor: Option<String>,
    },

    /// Reset configs
    Reset,
}
