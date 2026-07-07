use anyhow::Result;

pub struct SysProxyStatus {
    pub enabled: bool,
    pub proxy_port: Option<u16>,
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
        use windows::PlatformProxy;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        use linux::PlatformProxy;
    } else {
        mod unsupported;
        use unsupported::PlatformProxy;
    }
}

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