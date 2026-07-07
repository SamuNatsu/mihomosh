use anyhow::Result;

pub struct SysProxyStatus {
    pub enabled: bool,
    pub proxy_port: Option<u16>,
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::PlatformProxy;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::PlatformProxy;

pub struct SysProxyManager;

impl SysProxyManager {
    pub fn get() -> Result<SysProxyStatus> {
        PlatformProxy::get()
    }

    pub fn set(status: SysProxyStatus) -> Result<()> {
        PlatformProxy::set(status)
    }

    pub fn clear() -> Result<()> {
        PlatformProxy::clear()
    }
}