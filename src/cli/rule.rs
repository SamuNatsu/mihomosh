use clap::Subcommand;

#[derive(Subcommand)]
pub enum RuleCommand {
    /// List rules/rule sets
    ///
    /// By default, rules are listed
    List {
        /// List rule sets
        #[arg(short, long)]
        sets: bool,
    },

    /// Update specific/all rule set(s)
    ///
    /// If no options are provided, update all rule sets
    Update {
        /// Rule set name
        #[arg(short, long)]
        name: Option<String>,
    },
}
