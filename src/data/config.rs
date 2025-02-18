use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::OnceLock,
};

use serde::Deserialize;
use serde_yaml::Value;
use url::Url;

use crate::utils::{api::Api, path};

#[derive(Deserialize)]
pub struct Config {
    pub editor: String,
    pub mihomo_path: String,
    pub mihomo_api: Url,
    pub mihomo_token: Option<String>,
    pub mode: ConfigMode,
    pub mixed_port: u16,
    pub socks_port: Option<u16>,
    pub http_port: Option<u16>,

    #[serde(default)]
    pub allow_lan: bool,

    #[serde(default)]
    pub allow_ipv6: bool,

    pub extend_configs: Option<Value>,
    pub extend_scripts: Option<String>,
}
impl Config {
    pub const DEFAULT_CONFIG: &'static [u8] = include_bytes!("../includes/default_config.yaml");

    pub fn get_instance() -> &'static Self {
        static I: OnceLock<Config> = OnceLock::new();
        I.get_or_init(|| {
            let path = path::get_data_dir().join("config.yaml");
            if !path.is_file() {
                let mut file = File::create(&path).expect("fail to create config file");
                file.write_all(&Self::DEFAULT_CONFIG)
                    .expect("fail to write config file");
                file.flush().expect("fail to flush config file");
            }

            let contents = fs::read_to_string(&path).expect("fail to read config file");
            serde_yaml::from_str(&contents).expect("fail to parse config file")
        })
    }

    pub fn get_path(&self) -> PathBuf {
        path::get_data_dir().join("config.yaml")
    }

    pub fn get_api(&self) -> Api {
        Api::new(&self.mihomo_api, self.mihomo_token.clone())
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigMode {
    Direct,
    Rule,
    Global,
}
impl Display for ConfigMode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Direct => write!(f, "direct"),
            Self::Rule => write!(f, "rule"),
            Self::Global => write!(f, "global"),
        }
    }
}
