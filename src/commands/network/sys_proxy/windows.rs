use anyhow::{Context, Result};
use winreg::enums::{
    HKEY_CURRENT_USER,
    KEY_READ,
    KEY_WRITE,
};
use winreg::RegKey;

use super::SysProxyStatus;


pub struct PlatformProxy;


const PROXY_REG_PATH: &str =
    r"Software\Microsoft\Windows\CurrentVersion\Internet Settings";



impl PlatformProxy {

    /// 获取当前系统代理状态
    pub fn get() -> Result<SysProxyStatus> {

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);


        let settings = hkcu
            .open_subkey_with_flags(
                PROXY_REG_PATH,
                KEY_READ
            )
            .context(
                "Failed to open Internet Settings registry key"
            )?;


        let enabled: u32 = settings
            .get_value("ProxyEnable")
            .unwrap_or(0);


        let proxy_server: String = settings
            .get_value("ProxyServer")
            .unwrap_or_default();


        Ok(SysProxyStatus {
            enabled: enabled != 0,
            proxy_port: parse_port(&proxy_server),
        })
    }



    /// 设置系统代理
    pub fn set(status: SysProxyStatus) -> Result<()> {

        let port = status
            .proxy_port
            .unwrap_or(7890);


        let hkcu = RegKey::predef(HKEY_CURRENT_USER);


        let settings = hkcu
            .open_subkey_with_flags(
                PROXY_REG_PATH,
                KEY_WRITE
            )
            .context(
                "Failed to open Internet Settings registry key"
            )?;


        let proxy = format!(
            "127.0.0.1:{port}"
        );


        settings
            .set_value(
                "ProxyEnable",
                &1u32
            )
            .context(
                "Failed to enable proxy"
            )?;


        settings
            .set_value(
                "ProxyServer",
                &proxy
            )
            .context(
                "Failed to set ProxyServer"
            )?;


        settings
            .set_value(
                "ProxyOverride",
                &"localhost;127.0.0.1;<local>"
            )
            .context(
                "Failed to set ProxyOverride"
            )?;

        Ok(())
    }



    /// 禁用系统代理
    pub fn clear() -> Result<()> {

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);


        let settings = hkcu
            .open_subkey_with_flags(
                PROXY_REG_PATH,
                KEY_WRITE
            )
            .context(
                "Failed to open Internet Settings registry key"
            )?;


        settings
            .set_value(
                "ProxyEnable",
                &0u32
            )
            .context(
                "Failed to disable proxy"
            )?;
            
        Ok(())
    }
}

/// 从 ProxyServer 中解析端口
fn parse_port(server: &str) -> Option<u16> {

    let first = server
        .split(';')
        .next()?;


    let addr = first
        .split('=')
        .last()?;


    addr
        .rsplit(':')
        .next()?
        .parse()
        .ok()
}