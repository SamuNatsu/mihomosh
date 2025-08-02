use clap::Subcommand;

#[derive(Subcommand)]
pub enum RuleSetArgs {
    /// Print all rule sets
    View,

    /// Update a rule set / Update all rule sets
    Update {
        /// Rule set name, update all when not present
        name: Option<String>,
    },
}
