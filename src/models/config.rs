use std::{fs::File, path::PathBuf, sync::OnceLock};

use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use smart_default::SmartDefault;
use strum::AsRefStr;
use tinytemplate::TinyTemplate;
use url::Url;
use validator::Validate;

use crate::{templates, utils::dir};

#[derive(Deserialize, Serialize, SmartDefault, Validate)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[default = "/etc/mihomo/config.yaml"]
    pub mihomo_path: PathBuf,
    #[default(Url::parse("http://127.0.0.1:9090").unwrap())]
    pub mihomo_api: Url,
    pub mihomo_secret: Option<String>,
    pub mode: ConfigMode,
    pub allow_lan: bool,
    pub allow_ipv6: bool,
    #[default = true]
    pub unified_delay: bool,
    pub log_level: ConfigLogLevel,
    #[default = 7890]
    #[validate(range(min = 1))]
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
            let file = File::open(path)
                .wrap_err_with(|| format!("fail to open file `{}`", path.display()))
                .expect("configuration file should be readable");

            serde_json::from_reader(&file)
                .wrap_err_with(|| format!("fail to parse file `{}`", path.display()))
                .expect("configuration file should be valid")
        })
    }

    pub fn reset() -> Result<()> {
        let path = Self::get_path();
        let file = File::create(path)
            .wrap_err_with(|| format!("fail to create file `{}`", path.display()))?;

        serde_json::to_writer(&file, &Self::default())
            .wrap_err_with(|| format!("fail to serialize value to file `{}`", path.display()))
    }

    pub fn render(&self) -> Result<String> {
        let mut tt = TinyTemplate::new();
        tt.add_template("config", templates::CONFIG)
            .wrap_err("fail to add rendering template")?;
        tt.set_default_formatter(&|value, output| {
            if let Value::String(str) = &value {
                output.push_str(&serde_json::to_string(str)?);
                Ok(())
            } else {
                tinytemplate::format(value, output)
            }
        });

        Ok(tt.render("config", self)?)
    }
}
