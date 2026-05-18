use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Show configurations
    ///
    /// The file viewer used is determined by environment variable `PAGER`,
    /// with `less` serving as the fallback (on Windows, `more.com` is used).
    Show,

    /// Edit configurations
    ///
    /// The file editor used is determined by environment variables in the
    /// order `VISUAL` and `EDITOR`, with `vim` as the fallback (on Windows,
    /// `edit.exe` is used).
    Edit {
        /// Do not reactivate current profile
        #[arg(short = 'N', long)]
        no_reactivate: bool,
    },

    /// Reset configurations
    Reset {
        /// Do not reactivate current profile
        #[arg(short = 'N', long)]
        no_reactivate: bool,
    },
}
