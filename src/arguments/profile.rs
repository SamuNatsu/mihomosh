use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProfileArgs {
    /// Update a profile / Update all profiles
    Update {
        /// Profile UUID or name, update all when not present
        uuid_or_name: Option<String>,
    },

    /// Activate a profile / Reactivate last activated profile
    Activate {
        /// Profile UUID or name, reactivate when not present
        uuid_or_name: Option<String>,
    },

    /// Create a new profile
    Create {
        /// Editor to be used for creating
        #[arg(short, long)]
        editor: Option<String>,
    },

    /// Delete a profile
    Delete {
        /// Profile UUID or name
        uuid_or_name: String,
    },

    /// List profiles
    List,

    /// View a profile
    #[command(subcommand)]
    View(ProfileViewArgs),

    /// Edit a profile
    #[command(subcommand)]
    Edit(ProfileEditArgs),

    /// View global extend configs
    #[command(name = "vgxc")]
    ViewGlobalExtendConfig {
        /// Viewer to be used for viewing
        #[arg(short, long, default_value = "less")]
        viewer: String,
    },

    /// View global extend script
    #[command(name = "vgxs")]
    ViewGlobalExtendScript {
        /// Viewer to be used for viewing
        #[arg(short, long, default_value = "less")]
        viewer: String,
    },

    /// Edit global extend configs
    #[command(name = "egxc")]
    EditGlobalExtendConfig {
        /// Editor to be used for creating
        #[arg(short, long)]
        editor: Option<String>,
    },

    /// Edit global extend script
    #[command(name = "egxs")]
    EditGlobalExtendScript {
        /// Editor to be used for creating
        #[arg(short, long)]
        editor: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ProfileViewArgs {
    /// View a profile's info
    Info {
        /// Profile UUID or name
        uuid_or_name: String,

        /// Viewer to be used for viewing
        #[arg(short, long, default_value = "less")]
        viewer: String,
    },

    /// View a profile's file data
    File {
        /// Profile UUID or name
        uuid_or_name: String,

        /// Viewer to be used for viewing
        #[arg(short, long, default_value = "less")]
        viewer: String,
    },

    /// View a profile's extend configs
    ExtendConfig {
        /// Profile UUID or name
        uuid_or_name: String,

        /// Viewer to be used for viewing
        #[arg(short, long, default_value = "less")]
        viewer: String,
    },

    /// View a profile's extend script
    ExtendScript {
        /// Profile UUID or name
        uuid_or_name: String,

        /// Viewer to be used for viewing
        #[arg(short, long, default_value = "less")]
        viewer: String,
    },
}

#[derive(Subcommand)]
pub enum ProfileEditArgs {
    /// Edit a profile's info
    Info {
        /// Profile UUID or name
        uuid_or_name: String,

        /// Editor to be used for creating
        #[arg(short, long)]
        editor: Option<String>,
    },

    /// Edit a profile's file data
    File {
        /// Profile UUID or name
        uuid_or_name: String,

        /// Editor to be used for creating
        #[arg(short, long)]
        editor: Option<String>,
    },

    /// View a profile's extend configs
    ExtendConfig {
        /// Profile UUID or name
        uuid_or_name: String,

        /// Editor to be used for creating
        #[arg(short, long)]
        editor: Option<String>,
    },

    /// View a profile's extend script
    ExtendScript {
        /// Profile UUID or name
        uuid_or_name: String,

        /// Editor to be used for creating
        #[arg(short, long)]
        editor: Option<String>,
    },
}
