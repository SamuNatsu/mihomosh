use std::{fs::File, path::PathBuf, sync::OnceLock};

use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use url::Url;

use crate::utils::dir;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub mihomo_path: PathBuf,
    pub mihomo_api: Url,
    pub mihomo_secret: Option<String>,
    pub mode: ConfigMode,
    pub allow_lan: bool,
    pub allow_ipv6: bool,
    pub unified_delay: bool,
    pub log_level: ConfigLogLevel,
    pub port: u16,
}

#[derive(AsRefStr, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ConfigMode {
    #[default]
    Rule,
    Global,
    Direct,
}

#[derive(AsRefStr, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ConfigLogLevel {
    Silent,
    Error,
    Warning,
    #[default]
    Info,
    Debug,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mihomo_path: PathBuf::from("/etc/mihomo/config.yaml"),
            mihomo_api: Url::parse("http://127.0.0.1:9090").unwrap(),
            mihomo_secret: None,
            mode: ConfigMode::default(),
            allow_lan: false,
            allow_ipv6: false,
            unified_delay: true,
            log_level: ConfigLogLevel::default(),
            port: 7890,
        }
    }
}

impl Config {
    pub fn get_path() -> &'static PathBuf {
        static INSTANCE: OnceLock<PathBuf> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let path = dir::get_data_dir().join("config.json");
            if !path.is_file() {
                let file = File::create(&path)
                    .wrap_err_with(|| format!("fail to create file `{}`", path.display()))
                    .expect("configuration file should be writable");
                serde_json::to_writer(&file, &Self::default())
                    .wrap_err_with(|| {
                        format!("fail to serialize value to file `{}`", path.display())
                    })
                    .expect("default configuration serialization should be successful");
            }

            path
        })
    }

    pub fn get_instance() -> &'static Self {
        static INSTANCE: OnceLock<Config> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let path = Self::get_path();
            let file = File::open(&path)
                .wrap_err_with(|| format!("fail to open file `{}`", path.display()))
                .expect("configuration file should be readable");

            serde_json::from_reader(&file)
                .wrap_err_with(|| format!("fail to parse file `{}`", path.display()))
                .expect("configuration file should be valid")
        })
    }

    pub fn reset() -> Result<()> {
        let path = Self::get_path();
        let file = File::create(&path)
            .wrap_err_with(|| format!("fail to create file `{}`", path.display()))?;

        Ok(serde_json::to_writer(&file, &Self::default())
            .wrap_err_with(|| format!("fail to serialize value to file `{}`", path.display()))?)
    }
}
