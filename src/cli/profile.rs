use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum ProfileCommand {
    /// List profiles
    #[command(visible_alias = "ls")]
    List,

    /// Show specific profile information
    ///
    /// The file viewer used is determined by environment variable `PAGER`,
    /// with `less` serving as the fallback (on Windows, `more.com` is used).
    Show {
        #[command(flatten)]
        target: ProfileTarget,
    },

    /// Create new profile
    ///
    /// The file editor used is determined by environment variables in the
    /// order `VISUAL` and `EDITOR`, with `vim` as the fallback (on Windows,
    /// `edit.exe` is used).
    #[command(visible_alias = "new")]
    Create,

    /// Edit specific profile information
    ///
    /// The file editor used is determined by environment variables in the
    /// order `VISUAL` and `EDITOR`, with `vim` as the fallback (on Windows,
    /// `edit.exe` is used).
    Edit {
        #[command(flatten)]
        target: ProfileTarget,
    },

    /// Delete specific profile
    #[command(visible_alias = "rm")]
    Delete {
        #[command(flatten)]
        target: ProfileTarget,
    },

    /// Update specific/all profile(s)
    ///
    /// If neither `--uuid <UUID>` nor `--name <NAME>` is provided, update all
    /// profiles
    #[command(visible_alias = "upd")]
    Update {
        #[command(flatten)]
        target: ProfileOptionalTarget,

        /// Do not reactivate current profile if it is updated
        #[arg(short = 'N', long)]
        no_reactivate: bool,
    },

    /// Activate specific profile
    #[command(visible_alias = "use")]
    Activate {
        #[command(flatten)]
        target: ProfileTarget,
    },

    /// Reactivate last activated profile
    #[command(visible_alias = "reuse")]
    Reactivate,

    /// Manage profile data
    #[command(subcommand)]
    Data(ProfileDataCommand),

    /// Manage profile extensions
    #[command(subcommand, visible_alias = "ext")]
    Extension(ProfileExtensionCommand),
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct ProfileTarget {
    /// Profile UUID
    #[arg(short, long)]
    uuid: Option<String>,

    /// Profile name
    #[arg(short, long)]
    name: Option<String>,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
pub struct ProfileOptionalTarget {
    /// Profile UUID
    #[arg(short, long)]
    uuid: Option<String>,

    /// Profile name
    #[arg(short, long)]
    name: Option<String>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct ProfileGlobalTarget {
    /// Profile UUID
    #[arg(short, long)]
    uuid: Option<String>,

    /// Profile name
    #[arg(short, long)]
    name: Option<String>,

    /// Global
    #[arg(short, long)]
    global: bool,
}

#[derive(Subcommand)]
pub enum ProfileDataCommand {
    /// Show specific profile data
    ///
    /// The file viewer used is determined by environment variable `PAGER`,
    /// with `less` serving as the fallback (on Windows, `more.com` is used).
    Show {
        #[command(flatten)]
        target: ProfileTarget,
    },

    /// Edit specific profile data
    ///
    /// The file editor used is determined by environment variables in the
    /// order `VISUAL` and `EDITOR`, with `vim` as the fallback (on Windows,
    /// `edit.exe` is used).
    Edit {
        #[command(flatten)]
        target: ProfileTarget,

        /// Do not reactivate current profile if it is edited
        #[arg(short = 'N', long)]
        no_reactivate: bool,
    },
}

#[derive(Subcommand)]
pub enum ProfileExtensionCommand {
    /// Manage profile extend configurations
    #[command(subcommand, visible_alias = "conf")]
    Config(ProfileExtensionConfigCommand),

    /// Manage profile extend scripts
    #[command(subcommand, visible_alias = "scr")]
    Script(ProfileExtensionScriptCommand),
}

#[derive(Subcommand)]
pub enum ProfileExtensionConfigCommand {
    /// Show specific/global profile extend configurations
    ///
    /// The file viewer used is determined by environment variable `PAGER`,
    /// with `less` serving as the fallback (on Windows, `more.com` is used).
    Show {
        #[command(flatten)]
        target: ProfileGlobalTarget,
    },

    /// Edit specific/global profile extend configurations
    ///
    /// The file editor used is determined by environment variables in the
    /// order `VISUAL` and `EDITOR`, with `vim` as the fallback (on Windows,
    /// `edit.exe` is used).
    Edit {
        #[command(flatten)]
        target: ProfileGlobalTarget,

        /// Do not reactivate current profile if it is edited
        #[arg(short = 'N', long)]
        no_reactivate: bool,
    },
}

#[derive(Subcommand)]
pub enum ProfileExtensionScriptCommand {
    /// Show specific/global profile extend scripts
    ///
    /// The file viewer used is determined by environment variable `PAGER`,
    /// with `less` serving as the fallback (on Windows, `more.com` is used).
    Show {
        #[command(flatten)]
        target: ProfileGlobalTarget,
    },

    /// Edit specific/global profile extend scripts
    ///
    /// The file editor used is determined by environment variables in the
    /// order `VISUAL` and `EDITOR`, with `vim` as the fallback (on Windows,
    /// `edit.exe` is used).
    Edit {
        #[command(flatten)]
        target: ProfileGlobalTarget,

        /// Do not reactivate current profile if it is edited
        #[arg(short = 'N', long)]
        no_reactivate: bool,
    },
}
