use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use tokio::task::JoinSet;
use wildcard::Wildcard;

use crate::{
    arguments::connection::ConnectionArgs, models::config::Config, println_danger,
    println_secondary, println_success, println_warn, utils::api::resp::Connection,
};

pub async fn handle_connection(args: ConnectionArgs) -> Result<()> {
    match args {
        ConnectionArgs::View => view().await?,
        ConnectionArgs::Close {
            r#type,
            host,
            process,
            source,
            destination,
            chain,
            rule,
        } => close(r#type, host, process, source, destination, chain, rule).await?,
    }
    Ok(())
}

async fn view() -> Result<()> {
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
    if conns.is_empty() {
        println_secondary!("No connection");
        return Ok(());
    }

    // Print connection list
    for conn in conns {
        // Start time & matched rule
        let start = conn.get_start().context("Fail to get start time")?;
        println!(
            "Start: {}\tRule: {}",
            console::style(start).green(),
            console::style(conn.get_rule()).yellow()
        );

        // Type, source & destination
        println!(
            "Type: {}\t SRC: {}\tDST: {}",
            console::style(conn.get_type()).cyan(),
            console::style(conn.get_src()).magenta(),
            console::style(conn.get_dst()).magenta()
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

async fn close(
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
        if filter_by_type(&r#type, &conn)
            && filter_by_host(&host, &conn)
            && filter_by_process(&process, &conn)
            && filter_by_source(&source, &conn)
            && filter_by_destination(&destination, &conn)
            && filter_by_chain(&chain, &conn)
            && filter_by_rule(&rule, &conn)
        {
            filtered.push(conn);
        }
    }
    println_secondary!("{} connection(s) found", filtered.len());

    // If no connection found
    if filtered.is_empty() {
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
                    "Closed: process=`{}` dst=`{}`",
                    conn.metadata.process,
                    conn.get_dst()
                ),
                Err(err) => println_danger!(
                    "Fail: process=`{}` dst=`{}` err=`{}`",
                    conn.metadata.process,
                    conn.get_dst(),
                    err
                ),
            }
        });
    }
    tasks.join_all().await;

    // Success
    Ok(())
}

fn filter_by_type(r#type: &Option<Vec<String>>, conn: &Connection) -> bool {
    if let Some(t) = &r#type {
        t.iter().any(|v| *v.trim() == conn.get_type())
    } else {
        true
    }
}

fn filter_by_host(host: &Option<Vec<String>>, conn: &Connection) -> bool {
    if let Some(h) = &host {
        h.iter().any(|v| {
            let ret = Wildcard::new(v.trim().as_bytes())
                .map(|w| w.is_match(conn.metadata.host.as_bytes()));
            match ret {
                Ok(ret) => ret,
                Err(err) => {
                    println_warn!("{err:?}");
                    true
                }
            }
        })
    } else {
        true
    }
}

fn filter_by_process(process: &Option<Vec<String>>, conn: &Connection) -> bool {
    if let Some(p) = &process {
        p.iter().any(|v| v.trim() == conn.metadata.process)
    } else {
        true
    }
}

fn filter_by_source(source: &Option<Vec<String>>, conn: &Connection) -> bool {
    if let Some(h) = &source {
        h.iter().any(|v| {
            let ret =
                Wildcard::new(v.trim().as_bytes()).map(|w| w.is_match(conn.get_src().as_bytes()));
            match ret {
                Ok(ret) => ret,
                Err(err) => {
                    println_warn!("{err:?}");
                    true
                }
            }
        })
    } else {
        true
    }
}

fn filter_by_destination(destination: &Option<Vec<String>>, conn: &Connection) -> bool {
    if let Some(d) = &destination {
        d.iter().any(|v| {
            let ret =
                Wildcard::new(v.trim().as_bytes()).map(|w| w.is_match(conn.get_dst().as_bytes()));
            match ret {
                Ok(ret) => ret,
                Err(err) => {
                    println_warn!("{err:?}");
                    true
                }
            }
        })
    } else {
        true
    }
}

fn filter_by_chain(chain: &Option<Vec<String>>, conn: &Connection) -> bool {
    if let Some(c) = &chain {
        c.iter().any(|v| conn.chains.iter().any(|w| v.trim() == w))
    } else {
        true
    }
}

fn filter_by_rule(rule: &Option<Vec<String>>, conn: &Connection) -> bool {
    if let Some(r) = &rule {
        r.iter().any(|v| v.trim() == conn.get_rule())
    } else {
        true
    }
}
