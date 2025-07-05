use clap::Subcommand;

#[derive(Subcommand)]
pub enum RuleSetArgs {
    /// Print all rule sets
    GetAll,

    /// Update all rule sets
    UpdateAll,
}
