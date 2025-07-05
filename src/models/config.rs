use std::{
    fs::{self, File},
    path::PathBuf,
    sync::OnceLock,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::utils;

const DEFAULT_CONFIG_CONTENTS: &'static str = include_str!("../includes/default_config.yaml");

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub mihomo_path: String,
    pub mihomo_api: String,
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
            let path = utils::get_data_dir().join("config.json");
            if !path.is_file() {
                fs::write(&path, DEFAULT_CONFIG_CONTENTS).expect("fail to write config file");
            }

            path
        })
    }

    pub fn get_instance() -> &'static Config {
        static INSTANCE: OnceLock<Config> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let path = Self::get_path();
            let file = File::open(&path).expect("fail to open config file");

            serde_json::from_reader(&file).expect("fail to parse config file")
        })
    }

    pub fn verify<S: AsRef<str>>(contents: S) -> Result<()> {
        serde_yml::from_str::<Self>(contents.as_ref())?;
        Ok(())
    }

    pub fn reset() -> Result<()> {
        let path = Self::get_path();
        fs::write(&path, DEFAULT_CONFIG_CONTENTS)?;

        Ok(())
    }
}
