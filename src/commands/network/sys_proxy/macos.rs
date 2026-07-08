use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::{
    utils::dir,
    println_secondary,
};

use super::SysProxyStatus;


pub struct PlatformProxy;


impl PlatformProxy {
    pub fn get() -> Result<SysProxyStatus> {

        let http = std::env::var("http_proxy")
            .ok()
            .or_else(|| {
                std::env::var("HTTP_PROXY").ok()
            });


        let enabled = http.is_some();


        let proxy_port = http
            .and_then(|v| parse_port(&v));


        Ok(SysProxyStatus {
            enabled,
            proxy_port,
        })
    }


    pub fn set(status: SysProxyStatus) -> Result<()> {

        let port = status
            .proxy_port
            .unwrap_or(7890);


        write_scripts(
            port,
            port,
        );

        println_secondary!("Manually source the script to enable proxy. path:{dir::get_data_dir().display()}");
    }



    /// 清空代理脚本
    pub fn clear() -> Result<()> {

        clear_scripts()
    }
}



// =======================
// 文件路径
// =======================


fn script_paths() -> [PathBuf; 3] {

    let dir = dir::get_data_dir()
        .to_owned();


    [
        dir.join("proxy.sh"),
        dir.join("proxy.fish"),
        dir.join("proxy.elv"),
    ]
}



// =======================
// 写入脚本
// =======================


fn write_scripts(
    http_port: u16,
    https_port: u16,
) -> Result<()> {

    let [sh, fish, elv] = script_paths();


    fs::write(
        &sh,
        bash_script(
            http_port,
            https_port,
        ),
    )
    .with_context(|| {
        format!(
            "Fail to write `{}`",
            sh.display()
        )
    })?;


    fs::write(
        &fish,
        fish_script(
            http_port,
            https_port,
        ),
    )
    .with_context(|| {
        format!(
            "Fail to write `{}`",
            fish.display()
        )
    })?;


    fs::write(
        &elv,
        elvish_script(
            http_port,
            https_port,
        ),
    )
    .with_context(|| {
        format!(
            "Fail to write `{}`",
            elv.display()
        )
    })?;


    Ok(())
}



fn clear_scripts() -> Result<()> {

    let [sh, fish, elv] = script_paths();


    let empty =
        "# Mihomo proxy is OFF\n";


    for path in [sh, fish, elv] {

        fs::write(
            &path,
            empty,
        )
        .with_context(|| {
            format!(
                "Fail to write `{}`",
                path.display()
            )
        })?;
    }


    Ok(())
}



// =======================
// 工具函数
// =======================


fn parse_port(url: &str) -> Option<u16> {

    url.rsplit(':')
        .next()?
        .parse()
        .ok()
}



// =======================
// Shell脚本模板
// =======================


fn bash_script(
    http_port: u16,
    https_port: u16,
) -> String {

    format!(
r#"#!/bin/sh
# Mihomo proxy environment setup for macOS bash/zsh
#
# Enable:
#   source ~/.local/share/mihomosh/proxy.sh


export http_proxy="http://127.0.0.1:{http_port}"
export https_proxy="http://127.0.0.1:{https_port}"

export HTTP_PROXY="$http_proxy"
export HTTPS_PROXY="$https_proxy"


export all_proxy="socks5h://127.0.0.1:{http_port}"
export ALL_PROXY="$all_proxy"


export no_proxy="localhost,127.0.0.1,::1"
export NO_PROXY="$no_proxy"
"#
    )
}



fn fish_script(
    http_port: u16,
    https_port: u16,
) -> String {

    format!(
r#"# Mihomo proxy environment setup for fish
#
# Enable:
#   source ~/.local/share/mihomosh/proxy.fish


set -gx http_proxy "http://127.0.0.1:{http_port}"
set -gx https_proxy "http://127.0.0.1:{https_port}"

set -gx HTTP_PROXY $http_proxy
set -gx HTTPS_PROXY $https_proxy


set -gx all_proxy "socks5h://127.0.0.1:{http_port}"
set -gx ALL_PROXY $all_proxy


set -gx no_proxy "localhost,127.0.0.1,::1"
set -gx NO_PROXY $no_proxy
"#
    )
}



fn elvish_script(
    http_port: u16,
    https_port: u16,
) -> String {

    format!(
r#"# Mihomo proxy environment setup for elvish
#
# Enable:
#   eval (slurp < ~/.local/share/mihomosh/proxy.elv)


set E:http_proxy "http://127.0.0.1:{http_port}"
set E:https_proxy "http://127.0.0.1:{https_port}"

set E:HTTP_PROXY $E:http_proxy
set E:HTTPS_PROXY $E:https_proxy


set E:all_proxy "socks5h://127.0.0.1:{http_port}"
set E:ALL_PROXY $E:all_proxy


set E:no_proxy "localhost,127.0.0.1,::1"
set E:NO_PROXY $E:no_proxy
"#
    )
}