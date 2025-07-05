use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigArgs {
    /// Print all configs
    GetAll,

    /// Print specified config
    Get {
        /// Config key
        key: String,
    },

    /// Update specified config
    Set {
        /// Config key
        key: String,

        /// New config value
        value: String,
    },
}
