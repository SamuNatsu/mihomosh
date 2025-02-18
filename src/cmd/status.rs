use std::{cmp::Ordering, collections::HashMap, sync::LazyLock};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::{
    data::config::Config,
    utils::{file, result::normal},
};

pub static GROUP_TYPE_ORDERING: LazyLock<HashMap<String, i32>> = LazyLock::new(|| {
    HashMap::from([
        ("Selector".to_owned(), 0),
        ("URLTest".to_owned(), 1),
        ("Fallback".to_owned(), 2),
        ("LoadBalance".to_owned(), 3),
        ("Relay".to_owned(), 4),
    ])
});

pub async fn configs() -> Result<()> {
    // Call API
    let conf = Config::get_instance()
        .get_api()
        .get_configs()
        .await
        .with_context(|| "try to call api")?;
    let contents = serde_yaml::to_string(&conf).with_context(|| "try to convert to yaml")?;

    // Show configs
    file::show_contents("-configs.yaml", &contents, true).with_context(|| "try to show configs")?;

    // Success
    Ok(())
}

pub async fn connections() -> Result<()> {
    // Call API
    let value = Config::get_instance()
        .get_api()
        .get_connections()
        .await
        .with_context(|| "try to call api")?
        .as_object()
        .unwrap()
        .get("connections")
        .unwrap()
        .to_owned();
    if value.is_null() {
        return normal!("No connection");
    }

    // Parse data
    #[derive(Deserialize)]
    struct Conn {
        metadata: ConnMeta,
        start: String,
        chains: Vec<String>,
        rule: String,
    }
    #[derive(Deserialize)]
    struct ConnMeta {
        network: String,
        r#type: String,
        #[serde(alias = "sourceIP")]
        source_ip: String,
        #[serde(alias = "destinationIP")]
        destination_ip: String,
        #[serde(alias = "sourcePort")]
        source_port: String,
        #[serde(alias = "destinationPort")]
        destination_port: String,
        host: String,
    }
    let mut value =
        serde_json::from_value::<Vec<Conn>>(value).with_context(|| "try to parse data")?;
    value.sort_by_key(|v| v.start.to_owned());

    // Print connections
    for conn in value {
        println!(
            "[{}:{}] {}({}:{}) {} {}",
            console::style(conn.metadata.network).bright().green(),
            console::style(conn.metadata.r#type).bright().cyan(),
            if conn.metadata.host.len() == 0 {
                console::style("No host".to_owned()).bright().black()
            } else {
                console::style(conn.metadata.host).bold().bright().blue()
            },
            conn.metadata.destination_ip,
            conn.metadata.destination_port,
            console::style(format!(
                "<-[{}:{}]",
                conn.metadata.source_ip, conn.metadata.source_port
            ))
            .bright()
            .red(),
            console::style(conn.rule).bright().yellow()
        );

        let chains = conn.chains.iter().rev().fold("Local".to_owned(), |acc, c| {
            format!("{acc} -> {}", console::style(c).bright().magenta())
        });
        println!("  {chains}");
    }

    // Success
    Ok(())
}

pub async fn groups() -> Result<()> {
    // Call API
    let value = Config::get_instance()
        .get_api()
        .get_groups()
        .await
        .with_context(|| "try to call api")?
        .as_object()
        .unwrap()
        .get("proxies")
        .unwrap()
        .to_owned();

    // Parse data
    #[derive(Deserialize)]
    struct Group {
        name: String,
        now: String,
        r#type: String,
    }
    let mut groups =
        serde_json::from_value::<Vec<Group>>(value).with_context(|| "try to parse data")?;
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

    // Success
    Ok(())
}

pub async fn version() -> Result<()> {
    // Call API
    let version = Config::get_instance()
        .get_api()
        .get_version()
        .await
        .with_context(|| "try to call api")?;

    // Print version
    println!("{version}");

    // Success
    Ok(())
}
