use anyhow::{Context, Result, anyhow};
use tokio::task::JoinSet;

use crate::{
    arguments::rule_set::RuleSetArgs, models::config::Config, println_danger, println_success,
    str_primary, str_success, str_warn,
};

pub async fn handle_rule_set(args: RuleSetArgs) -> Result<()> {
    match args {
        RuleSetArgs::View => view().await?,
        RuleSetArgs::Update { name } => update(name).await?,
    }
    Ok(())
}

async fn view() -> Result<()> {
    let mut rule_sets = Config::get_instance()
        .get_api()
        .get_rule_sets()
        .await
        .context("Fail to get rule sets")?
        .into_values()
        .collect::<Vec<_>>();
    rule_sets.sort_by_key(|rule_set| rule_set.name.clone());

    for rule_set in rule_sets {
        println!(
            "{}({}) [{}:{}]",
            str_primary!("{}", rule_set.name),
            rule_set.behavior,
            str_success!("{}", rule_set.vehicle_type),
            str_warn!("{}", rule_set.r#type)
        );
    }

    Ok(())
}

async fn update(name: Option<String>) -> Result<()> {
    let rule_sets = Config::get_instance()
        .get_api()
        .get_rule_sets()
        .await
        .context("Fail to get rule sets")?;

    let rule_sets = if let Some(name) = name {
        let ret = rule_sets
            .get(&name)
            .ok_or(anyhow!("Name not found"))
            .context("Fail to get rule set")?
            .clone();
        vec![ret]
    } else {
        rule_sets.into_values().collect::<Vec<_>>()
    };

    // Create tasks
    let mut join_set = JoinSet::new();
    for rule_set in rule_sets {
        join_set.spawn(async {
            let name = rule_set.name;
            let res: Result<()> = async {
                Config::get_instance()
                    .get_api()
                    .update_rule_set(&name)
                    .await
                    .with_context(|| format!("Fail to update rule set `{name}`"))?;
                Ok(())
            }
            .await;

            println_success!("Rule set `{name}` updated");
            (name, res)
        });
    }

    // Print errors
    let res = join_set.join_all().await;
    let errs = res
        .iter()
        .filter_map(|(_, res)| res.as_ref().err())
        .collect::<Vec<_>>();
    for err in errs {
        println_danger!("{err:?}");
    }

    Ok(())
}
