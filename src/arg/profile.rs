use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Activate a profile
    Activate {
        /// Profile UUID or name
        uuid_or_name: String,
    },

    /// Delete a profile
    #[command(visible_aliases = ["del", "rm"])]
    Delete {
        /// Profile UUID or name
        uuid_or_name: String,
    },

    /// Edit a profile's configurations
    EditConfigs {
        /// Profile UUID or name
        uuid_or_name: String,
    },

    /// Edit a profile's data
    EditData {
        /// Profile UUID or name
        uuid_or_name: String,
    },

    /// List profiles
    #[command(visible_alias = "ls")]
    List,

    /// Create a new profile
    #[command(visible_alias = "add")]
    New,

    /// Update a profile or all profiles
    #[command(visible_alias = "up")]
    Update {
        /// Profile UUID or name
        uuid_or_name: Option<String>,
    },

    /// View a profile's configurations
    ViewConfigs {
        /// Profile UUID or name
        uuid_or_name: String,
    },

    /// View a profile's raw data contents
    ViewData {
        /// Profile UUID or name
        uuid_or_name: String,
    },

    /// View a profile's rules
    ViewRules {
        /// Profile UUID or name
        uuid_or_name: String,
    },
}
