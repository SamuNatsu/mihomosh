use anyhow::{bail, Result};

use crate::println_warn;
use super::SysProxyStatus;

pub struct PlatformProxy;

impl PlatformProxy {
    pub fn get() -> Result<SysProxyStatus> {
        println_warn!("Current platform is not supported, operation is invalid");
        bail!("unsupported platform")
    }

    pub fn set(_status: SysProxyStatus) -> Result<()> {
        println_warn!("Current platform is not supported, operation is invalid");
        bail!("unsupported platform")
    }

    pub fn clear() -> Result<()> {
        println_warn!("Current platform is not supported, operation is invalid");
        bail!("unsupported platform")
    }
}
