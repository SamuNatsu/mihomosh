use anyhow::{Context, Result, bail};
use futures::StreamExt;

use crate::{
    arguments::inspect::InspectArgs, models::config::Config, println_primary, println_success,
};

pub async fn handle_inspect(args: InspectArgs) -> Result<()> {
    match args {
        InspectArgs::Log => log().await?,
        InspectArgs::Traffic => traffic().await?,
        InspectArgs::Memory => memory().await?,
        InspectArgs::Version => version().await?,
    }
    Ok(())
}

async fn log() -> Result<()> {
    println_primary!("Start tracing Mihomo logs...");

    // Create stream
    let api = Config::get_instance().get_api();
    let mut stream = api.get_logs().await.context("Fail to get log stream")?;

    // Parse logs
    while let Some(log) = stream.next().await {
        let (t, p) = log.context("Fail to extract log data")?;
        match t.as_str() {
            "info" => println!("{} {p}", console::style("[INFO ]").green()),
            "warning" => println!("{} {p}", console::style("[WARN ]").yellow()),
            "error" => println!("{} {p}", console::style("[ERROR]").red()),
            "debug" => println!("{} {p}", console::style("[DEBUG]").blue()),
            _ => bail!("Unexpected log type `{t}`"),
        }
    }

    // Success
    Ok(())
}

async fn traffic() -> Result<()> {
    println_primary!("Start tracing Mihomo traffic...");

    // Create stream
    let api = Config::get_instance().get_api();
    let mut stream = api
        .get_traffic()
        .await
        .context("Fail to get traffic stream")?;

    // Parse logs
    while let Some(traffic) = stream.next().await {
        let (up, down) = traffic.context("Fail to extract traffic data")?;
        println!(
            "Up: {}\t\tDown: {}",
            get_colored_number(up, "/s"),
            get_colored_number(down, "/s")
        )
    }

    // Success
    Ok(())
}

async fn memory() -> Result<()> {
    println_primary!("Start tracing Mihomo memory usage...");

    // Create stream
    let api = Config::get_instance().get_api();
    let mut stream = api
        .get_memory()
        .await
        .context("Fail to get memory stream")?;

    // Parse logs
    while let Some(memory) = stream.next().await {
        let inuse = memory.context("Fail to extract memory data")?;
        println!("Inuse: {}", get_colored_number(inuse, ""))
    }

    // Success
    Ok(())
}

async fn version() -> Result<()> {
    let version = Config::get_instance()
        .get_api()
        .get_version()
        .await
        .context("Fail to get version")?;

    println_success!("Mihomo version: {version}");
    Ok(())
}

fn get_colored_number<S: AsRef<str>>(num: u64, suffix: S) -> String {
    let text = if num < 1024 {
        format!("{num} B{}", suffix.as_ref())
    } else if num < 1048576 {
        format!("{:.1} KB{}", num as f64 / 1024.0, suffix.as_ref())
    } else {
        format!("{:.1} MB{}", num as f64 / 1048576.0, suffix.as_ref())
    };

    console::style(text).green().to_string()
}
