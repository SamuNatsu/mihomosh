use std::{
    fs::File,
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use smart_default::SmartDefault;
use strum::AsRefStr;
use tinytemplate::TinyTemplate;
use url::Url;
use validator::Validate;

use crate::templates;

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
    fn get_path() -> &'static PathBuf {
        static INSTANCE: LazyLock<PathBuf> = LazyLock::new(|| {
            // Get configuration file path
            let path = super::DATA_LOCAL_DIR.join("config.json");

            // Create default configuration file if not exists
            if !path.is_file() {
                // Create empty file
                let file = File::create(&path)
                    .wrap_err_with(|| format!("failed to create file `{}`", path.display()))
                    .expect("configuration file should be writable");

                // Write default configurations
                serde_json::to_writer(&file, &Config::default())
                    .wrap_err_with(|| {
                        format!("failed to serialize value to file `{}`", path.display())
                    })
                    .expect("default configuration serialization should be successful");
            }

            // Return path
            path
        });
        &INSTANCE
    }

    pub fn get_instance() -> &'static Mutex<Self> {
        pub static INSTANCE: LazyLock<Mutex<Config>> = LazyLock::new(|| {
            // Open configuration file
            let path = Config::get_path();
            let file = File::open(path)
                .wrap_err_with(|| format!("failed to open file `{}`", path.display()))
                .expect("configuration file should be readable");

            // Parse configurations
            let config = serde_json::from_reader(&file)
                .wrap_err_with(|| format!("failed to parse file `{}`", path.display()))
                .expect("configuration file should be valid");

            // Wrap mutex
            Mutex::new(config)
        });
        &INSTANCE
    }

    pub fn commit(new: Self) -> Result<()> {
        // Create file
        let path = Self::get_path();
        let file = File::create(path)
            .wrap_err_with(|| format!("failed to create file `{}`", path.display()))?;

        // Write new configurations
        serde_json::to_writer(&file, &new)
            .wrap_err_with(|| format!("failed to serialize value to file `{}`", path.display()))?;

        // Update instance
        *Self::get_instance().lock().unwrap() = new;

        // Done
        Ok(())
    }

    pub fn reset() -> Result<()> {
        Self::commit(Self::default())
    }

    pub fn render(&self) -> Result<String> {
        // Setup template engine
        let mut tt = TinyTemplate::new();
        tt.add_template("config", templates::CONFIG)
            .wrap_err("failed to add rendering template")?;
        tt.set_default_formatter(&|value, output| {
            if let Value::String(str) = &value {
                output.push_str(&serde_json::to_string(str)?);
                Ok(())
            } else {
                tinytemplate::format(value, output)
            }
        });

        // Render configurations
        Ok(tt.render("config", self)?)
    }
}
