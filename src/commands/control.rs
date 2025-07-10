use anyhow::{Context, Result};

use crate::{models::config::Config, println_success};

pub async fn flush_cache() -> Result<()> {
    Config::get_instance()
        .get_api()
        .flush_fake_ip_cache()
        .await
        .context("Fail to flush fake IP cache")?;

    println_success!("Mihomo fake IP cache flushed");
    Ok(())
}

pub async fn update_ui() -> Result<()> {
    Config::get_instance()
        .get_api()
        .upgrade_ui()
        .await
        .context("Faile to update UI")?;

    println_success!("External UI updated");
    Ok(())
}

pub async fn update_geo() -> Result<()> {
    Config::get_instance()
        .get_api()
        .upgrade_geo()
        .await
        .context("Fail to update GEO database")?;

    println_success!("Mihomo GEO database updated");
    Ok(())
}

pub async fn restart() -> Result<()> {
    Config::get_instance()
        .get_api()
        .restart()
        .await
        .context("Fail to restart Mihomo")?;

    println_success!("Mihomo restarted");
    Ok(())
}
