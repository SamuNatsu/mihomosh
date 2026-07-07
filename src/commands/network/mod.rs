mod sys_proxy;

use anyhow::{Context, Result};
use crate::{
    arguments::network::NetworkArgs,
    models::config::Config,
    println_secondary, println_success,
};

use sys_proxy::{SysProxyManager, SysProxyStatus};

pub async fn handle_network(args: NetworkArgs) -> Result<()> {
    match args {
        NetworkArgs::View => view().await?,
        NetworkArgs::Proxy => set_proxy().await?,
        NetworkArgs::UnProxy => clear_proxy().await?,
    }
    Ok(())
}

async fn view() -> Result<()> {
    let status = SysProxyManager::get()?;
    if status.enabled {
        println_success!(
            "System proxy is ON (port: {})",
            status.proxy_port.unwrap_or(0)
        );
    } else {
        println_secondary!("System proxy is OFF");
    }
    Ok(())
}

async fn set_proxy() -> Result<()> {
    let cfg = Config::get_instance();
    let port = cfg.mixed_port.or(cfg.port).unwrap_or(7890);

    SysProxyManager::set(SysProxyStatus {
        enabled: true,
        proxy_port: Some(port),
    })
    .context("Fail to set system proxy")?;

    println_success!("System proxy enabled on port {port}");
    Ok(())
}

async fn clear_proxy() -> Result<()> {
    SysProxyManager::clear().context("Fail to clear system proxy")?;
    println_success!("System proxy disabled");
    Ok(())
}