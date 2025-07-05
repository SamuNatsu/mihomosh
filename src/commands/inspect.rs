use anyhow::{Result, bail};
use futures::StreamExt;

use crate::{models::config::Config, success};

pub async fn log() -> Result<()> {
    println!(
        "{}",
        console::style("Start tracing Mihomo logs...")
            .bold()
            .bright()
            .green()
    );

    // Create stream
    let api = Config::get_instance().get_api();
    let mut stream = api.get_logs().await?;

    // Parse logs
    while let Some(inuse) = stream.next().await {
        let (t, p) = inuse?;
        match t.as_str() {
            "info" => println!("{} {}", console::style("[INFO ]").green(), p),
            "warning" => println!("{} {}", console::style("[WARN ]").yellow(), p),
            "error" => println!("{} {}", console::style("[ERROR]").red(), p),
            "debug" => println!("{} {}", console::style("[DEBUG]").blue(), p),
            _ => bail!("unexpected log type `{t}`"),
        }
    }

    // Success
    Ok(())
}

pub async fn traffic() -> Result<()> {
    println!(
        "{}",
        console::style("Start tracing Mihomo traffic...")
            .bold()
            .bright()
            .green()
    );

    // Create stream
    let api = Config::get_instance().get_api();
    let mut stream = api.get_traffic().await?;

    // Parse logs
    while let Some(inuse) = stream.next().await {
        let (up, down) = inuse?;
        println!(
            "Up: {}\t\tDown: {}",
            get_colored_number(up, "/s"),
            get_colored_number(down, "/s")
        )
    }

    // Success
    Ok(())
}

pub async fn memory() -> Result<()> {
    println!(
        "{}",
        console::style("Start tracing Mihomo memory usage...")
            .bold()
            .bright()
            .green()
    );

    // Create stream
    let api = Config::get_instance().get_api();
    let mut stream = api.get_memory().await?;

    // Parse logs
    while let Some(inuse) = stream.next().await {
        let inuse = inuse?;
        println!("Inuse: {}", get_colored_number(inuse, ""))
    }

    // Success
    Ok(())
}

pub async fn version() -> Result<()> {
    let version = Config::get_instance().get_api().get_version().await?;
    success!("Mihomo version: {version}")
}

fn get_colored_number<S: AsRef<str>>(num: u64, suffix: S) -> String {
    let text = if num < 1024 {
        format!("{} B{}", num, suffix.as_ref())
    } else if num < 1048576 {
        format!("{:.1} KB{}", num as f64 / 1024.0, suffix.as_ref())
    } else {
        format!("{:.1} MB{}", num as f64 / 1048576.0, suffix.as_ref())
    };

    console::style(text).green().to_string()
}
