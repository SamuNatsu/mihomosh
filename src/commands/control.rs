use anyhow::Result;

use crate::{models::config::Config, println_success};

pub async fn flush_cache() -> Result<()> {
    Config::get_instance()
        .get_api()
        .flush_fake_ip_cache()
        .await?;
    println_success!("Mihomo fake IP cache flushed");
    Ok(())
}

pub async fn update_ui() -> Result<()> {
    Config::get_instance().get_api().upgrade_ui().await?;
    println_success!("External UI updated");
    Ok(())
}

pub async fn update_geo() -> Result<()> {
    Config::get_instance().get_api().upgrade_geo().await?;
    println_success!("Mihomo GEO database updated");
    Ok(())
}

pub async fn restart() -> Result<()> {
    Config::get_instance().get_api().restart().await?;
    println_success!("Mihomo restarted");
    Ok(())
}
