use std::{
    fs::{self, File},
    path::PathBuf,
    sync::OnceLock,
};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{includes::DEFAULT_CONFIG_TEMPLATE, utils::dir};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub mihomo_path: PathBuf,
    pub mihomo_api: Url,
    pub mihomo_secret: Option<String>,
    pub log_level: ConfigLogLevel,
    pub mode: ConfigMode,
    pub port: Option<u16>,
    pub socks_port: Option<u16>,
    pub mixed_port: Option<u16>,
    pub allow_lan: bool,
    pub allow_ipv6: bool,
    pub unified_delay: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigLogLevel {
    Silent,
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigMode {
    Rule,
    Global,
    Direct,
}

impl Config {
    pub fn get_path() -> &'static PathBuf {
        static INSTANCE: OnceLock<PathBuf> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let path = dir::get_data_dir().join("config.json");
            if !path.is_file() {
                fs::write(&path, DEFAULT_CONFIG_TEMPLATE)
                    .with_context(|| format!("Fail to write file `{}`", path.display()))
                    .unwrap();
            }

            path
        })
    }

    pub fn get_instance() -> &'static Config {
        static INSTANCE: OnceLock<Config> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let path = Self::get_path();
            let file = File::open(path)
                .with_context(|| format!("Fail to open file `{}`", path.display()))
                .unwrap();

            serde_yml::from_reader(&file)
                .with_context(|| format!("Fail to parse config file `{}`", path.display()))
                .unwrap()
        })
    }

    pub fn verify(&self) -> Result<()> {
        if self.port == Some(0) {
            bail!("`port` cannot be 0");
        }
        if self.socks_port == Some(0) {
            bail!("`socks-port` cannot be 0");
        }
        if self.mixed_port == Some(0) {
            bail!("`mixed-port` cannot be 0");
        }
        Ok(())
    }

    pub fn reset() -> Result<()> {
        let path = Self::get_path();
        fs::write(path, DEFAULT_CONFIG_TEMPLATE)
            .with_context(|| format!("Fail to write file `{}`", path.display()))?;

        Ok(())
    }
}
