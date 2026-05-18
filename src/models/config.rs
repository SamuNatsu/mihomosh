use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use url::Url;

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

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConfigMode {
    Rule,
    Global,
    Direct,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConfigLogLevel {
    Silent,
    Error,
    Warning,
    Info,
    Debug,
}
