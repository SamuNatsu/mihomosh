use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use tokio::task::JoinSet;
use wildcard::Wildcard;

use crate::{
    models::config::Config, println_danger, println_secondary, println_success, style_fmt,
};

pub async fn view() -> Result<()> {
    // Get & sort connections
    let mut conns = Config::get_instance()
        .get_api()
        .get_connections()
        .await
        .context("Fail to get connections")?;
    conns.sort_by_key(|v| {
        v.start
            .parse::<DateTime<Utc>>()
            .map_or(0, |v| v.timestamp_millis())
    });

    // If no connection
    if conns.len() == 0 {
        println_secondary!("No connection");
        return Ok(());
    }

    // Print connection list
    for conn in conns {
        // Start time & matched rule
        let start = conn
            .start
            .parse::<DateTime<Local>>()?
            .format("%Y-%m-%dT%H:%M:%S.%3f%:z")
            .to_string();
        println!(
            "Start: {}\tRule: {}",
            console::style(start).green(),
            style_fmt!(
                "{}{}",
                conn.rule,
                if conn.rule_payload.is_empty() {
                    "".to_owned()
                } else {
                    format!("({})", conn.rule_payload)
                }
            )
            .yellow()
        );

        // Type, source & destination
        println!(
            "Type: {}\t SRC: {}\tDST: {}",
            style_fmt!("{}({})", conn.metadata.r#type, conn.metadata.network).cyan(),
            style_fmt!("{}:{}", conn.metadata.source_ip, conn.metadata.source_port).magenta(),
            style_fmt!(
                "{}:{}",
                conn.metadata.destination_ip,
                conn.metadata.destination_port
            )
            .magenta()
        );

        // Host & process
        println!(
            "Host: {}\tProcess: {}",
            console::style(conn.metadata.host).blue(),
            console::style(conn.metadata.process).blue()
        );

        // Chains
        let chains = conn
            .chains
            .iter()
            .map(|s| console::style(s).on_white().black().to_string())
            .collect::<Vec<_>>()
            .join(" -> ");
        println!("Chains: {chains}\n");
    }

    // Success
    Ok(())
}

pub async fn close(
    r#type: Option<Vec<String>>,
    host: Option<Vec<String>>,
    process: Option<Vec<String>>,
    source: Option<Vec<String>>,
    destination: Option<Vec<String>>,
    chain: Option<Vec<String>>,
    rule: Option<Vec<String>>,
) -> Result<()> {
    // Close all
    if r#type.is_none()
        && host.is_none()
        && process.is_none()
        && source.is_none()
        && destination.is_none()
        && chain.is_none()
        && rule.is_none()
    {
        Config::get_instance()
            .get_api()
            .close_all_connections()
            .await
            .context("Fail to close all connections")?;

        println_success!("All connection closed");
        return Ok(());
    }

    // Filter connections
    let api = Config::get_instance().get_api();
    let conns = api
        .get_connections()
        .await
        .context("Fail to get connections")?;
    let mut filtered = Vec::new();

    for conn in conns {
        // Type
        if let Some(t) = &r#type {
            if !t.iter().any(|v| {
                v.trim() == &format!("{}({})", conn.metadata.r#type, conn.metadata.network)
            }) {
                continue;
            }
        }

        // Host
        if let Some(h) = &host {
            if !h.iter().any(|v| {
                Wildcard::new(v.trim().as_bytes())
                    .and_then(|w| Ok(w.is_match(conn.metadata.host.as_bytes())))
                    .unwrap_or(false)
            }) {
                continue;
            }
        }

        // Process
        if let Some(p) = &process {
            if !p.iter().any(|v| v.trim() == conn.metadata.process) {
                continue;
            }
        }

        // Source
        if let Some(h) = &source {
            if !h.iter().any(|v| {
                Wildcard::new(v.trim().as_bytes())
                    .and_then(|w| {
                        Ok(w.is_match(
                            format!("{}:{}", conn.metadata.source_ip, conn.metadata.source_port)
                                .as_bytes(),
                        ))
                    })
                    .unwrap_or(false)
            }) {
                continue;
            }
        }

        // Destination
        if let Some(d) = &destination {
            if !d.iter().any(|v| {
                Wildcard::new(v.trim().as_bytes())
                    .and_then(|w| {
                        Ok(w.is_match(
                            format!(
                                "{}:{}",
                                conn.metadata.destination_ip, conn.metadata.destination_port
                            )
                            .as_bytes(),
                        ))
                    })
                    .unwrap_or(false)
            }) {
                continue;
            }
        }

        // Chain
        if let Some(c) = &chain {
            if !c.iter().any(|v| conn.chains.iter().any(|w| v.trim() == w)) {
                continue;
            }
        }

        // Rule
        if let Some(r) = &rule {
            let rule = format!(
                "{}{}",
                conn.rule,
                if conn.rule_payload.is_empty() {
                    "".to_owned()
                } else {
                    format!("({})", conn.rule_payload)
                }
            );
            if !r.iter().any(|v| v.trim() == &rule) {
                continue;
            }
        }

        // Found
        filtered.push(conn);
    }

    println_secondary!("{} connection(s) found", filtered.len());

    // If no connection found
    if filtered.len() == 0 {
        println_success!("No connection to be closed");
        return Ok(());
    }

    // Async close connections
    let mut tasks = JoinSet::new();
    for conn in filtered {
        tasks.spawn(async move {
            let ret = Config::get_instance()
                .get_api()
                .close_connection(&conn.id)
                .await;
            match ret {
                Ok(_) => println_success!(
                    "Closed: process=`{}` dst=`{}:{}`",
                    conn.metadata.process,
                    if conn.metadata.host.is_empty() {
                        conn.metadata.destination_ip
                    } else {
                        conn.metadata.host
                    },
                    conn.metadata.destination_port
                ),
                Err(err) => println_danger!(
                    "Fail: process=`{}` dst=`{}:{}` err=`{}`",
                    conn.metadata.process,
                    if conn.metadata.host.is_empty() {
                        conn.metadata.destination_ip
                    } else {
                        conn.metadata.host
                    },
                    conn.metadata.destination_port,
                    err
                ),
            }
        });
    }
    tasks.join_all().await;

    // Success
    Ok(())
}
