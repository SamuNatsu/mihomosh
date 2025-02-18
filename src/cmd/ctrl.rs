use std::cmp::Ordering;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use crate::{
    cmd::status::GROUP_TYPE_ORDERING,
    data::config::Config,
    utils::{prompt, result::success},
};

pub async fn update_geo() -> Result<()> {
    // Call API
    Config::get_instance()
        .get_api()
        .upgrade_geo()
        .await
        .with_context(|| "try to call api")?;

    // Success
    success!("GEO database updated")
}

pub async fn update_group() -> Result<()> {
    // Get groups
    let groups = Config::get_instance()
        .get_api()
        .get_groups()
        .await
        .with_context(|| "try to call api")?
        .as_object()
        .unwrap()
        .get("proxies")
        .unwrap()
        .to_owned();

    // Parse groups
    #[derive(Deserialize)]
    struct Group {
        all: Vec<String>,
        name: String,
        now: String,
        r#type: String,
    }
    let mut groups =
        serde_json::from_value::<Vec<Group>>(groups).with_context(|| "try to parse data")?;
    groups.sort_by(|a, b| {
        if a.name == "GLOBAL" {
            Ordering::Less
        } else if b.name == "GLOBAL" {
            Ordering::Greater
        } else if a.r#type != b.r#type {
            GROUP_TYPE_ORDERING
                .get(&a.r#type)
                .unwrap_or(&5)
                .cmp(GROUP_TYPE_ORDERING.get(&b.r#type).unwrap_or(&5))
        } else {
            a.name.cmp(&b.name)
        }
    });

    // Print groups
    let w = groups.len().to_string().len();
    for (idx, group) in groups.iter().enumerate() {
        println!(
            "[{:>w$}] {} ({}) -> {}",
            idx + 1,
            console::style(group.name.to_owned()).bold().bright().blue(),
            console::style(group.r#type.to_owned()).bright().yellow(),
            console::style(group.now.to_owned()).bold().bright().red(),
        );
    }

    // Ask
    let input = prompt::ask(
        console::style("Which group do you want to update? (Input group number): ")
            .bold()
            .bright()
            .cyan()
            .to_string(),
    )
    .with_context(|| "try to show ask prompt")?;
    let idx = input.trim().parse::<usize>()?;
    let group = groups.get_mut(idx - 1).ok_or(anyhow!("Out of index"))?;
    group.all.sort_by(|a, b| {
        if a == "GLOBAL" {
            Ordering::Less
        } else if b == "GLOBAL" {
            Ordering::Greater
        } else {
            a.cmp(b)
        }
    });

    // Print group selections
    let w = group.all.len().to_string().len();
    for (idx, sel) in group.all.iter().enumerate() {
        println!(
            "[{:>w$}] {}",
            idx + 1,
            console::style(sel.to_owned()).bold().bright().blue(),
        );
    }

    // Ask
    let input = prompt::ask(
        console::style("Which proxy do you want to choose? (Input proxy number): ")
            .bold()
            .bright()
            .cyan()
            .to_string(),
    )
    .with_context(|| "try to show ask prompt")?;
    let idx = input.trim().parse::<usize>()?;
    let proxy = group.all.get(idx - 1).ok_or(anyhow!("Out of range"))?;

    // Update
    Config::get_instance()
        .get_api()
        .update_proxy(&group.name, proxy)
        .await?;

    // Success
    success!("Group updated")
}

pub async fn restart() -> Result<()> {
    // Call API
    Config::get_instance()
        .get_api()
        .restart()
        .await
        .with_context(|| "try to call api")?;

    // Success
    success!("Mihomo restarted")
}
