use std::{fs, time::Duration};

use anyhow::{anyhow, bail, Result};
use boa_engine::{js_string, property::Attribute, vm::RuntimeLimits, Context, Source};
use boa_runtime::RegisterOptions;
use reqwest::Client;
use serde::Deserialize;
use serde_yaml::Value;
use url::Url;

use crate::{data::config::Config as AppConfig, utils::path};

#[derive(Clone, Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub uuid: String,

    pub name: String,
    pub r#type: ConfigType,
    pub url: Option<Url>,
    pub user_agent: Option<String>,

    #[serde(default)]
    pub use_proxy: ConfigProxy,

    #[serde(default)]
    pub allow_invalid_certs: bool,

    #[serde(default)]
    pub extend_configs: Option<Value>,

    #[serde(default)]
    pub extend_scripts: Option<String>,
}
impl Config {
    pub fn verify(&self) -> Result<()> {
        if self.name.trim().len() == 0 {
            bail!("profile name cannot be empty");
        }

        if self.r#type == ConfigType::Remote && (self.url.is_none() || self.user_agent.is_none()) {
            bail!("profile URL and user agent cannot be empty when type is `remote`");
        }

        if let Some(v) = &self.extend_configs {
            if !v.is_mapping() {
                bail!("profile extend configs must be an object");
            }
        }

        Ok(())
    }

    pub async fn fetch(&self) -> Result<(Option<usize>, Option<usize>, Option<i64>)> {
        // Check type
        if self.r#type != ConfigType::Remote {
            bail!("cannot fetch on a local profile");
        }

        // Fetch data
        let mut builder = Client::builder()
            .danger_accept_invalid_certs(self.allow_invalid_certs)
            .timeout(Duration::from_secs(30));
        match self.use_proxy {
            ConfigProxy::None => builder = builder.no_proxy(),
            ConfigProxy::System => (),
            ConfigProxy::Mihomo => todo!(),
        }
        let r = builder
            .build()?
            .get(self.url.clone().unwrap())
            .header("User-Agent", self.user_agent.clone().unwrap())
            .send()
            .await?;

        // Parse header
        let mut used = None;
        let mut total = None;
        let mut expired_at = None;
        if let Some(h) = r.headers().get("Subscription-Userinfo") {
            for seg in h.to_str()?.split(';') {
                if let Some((k, v)) = seg.trim().split_once('=') {
                    let v = match v.parse::<usize>() {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                    match k {
                        "upload" => {
                            used = if let Some(tmp) = used {
                                Some(tmp + v)
                            } else {
                                Some(v)
                            }
                        }
                        "download" => {
                            used = if let Some(tmp) = used {
                                Some(tmp + v)
                            } else {
                                Some(v)
                            }
                        }
                        "total" => total = Some(v),
                        "expire" => expired_at = Some(v as i64),
                        _ => (),
                    }
                }
            }
        }

        // Save file
        let path = path::get_profile_data_dir().join(format!("{}.yaml", self.uuid));
        fs::write(&path, r.text().await?)?;

        // Success
        Ok((used, total, expired_at))
    }

    pub async fn activate(&self) -> Result<()> {
        let cfg = AppConfig::get_instance();

        // Load data
        let path = path::get_profile_data_dir().join(format!("{}.yaml", self.uuid));
        let contents = if path.is_file() {
            fs::read_to_string(&path)?
        } else {
            String::new()
        };
        let mut value = serde_yaml::from_str::<Value>(&contents)?;

        // Merge mihomosh configs
        let mut buf = format!(
            "mode: {}\nallow-lan: {}\nipv6: {}\nmixed-port: {}\n",
            cfg.mode, cfg.allow_lan, cfg.allow_ipv6, cfg.mixed_port
        );
        if let Some(port) = &cfg.socks_port {
            buf.push_str(&format!("socks-port: {}\n", port));
        }
        if let Some(port) = &cfg.http_port {
            buf.push_str(&format!("port: {}\n", port));
        }
        merge_yaml(&serde_yaml::from_str(&buf)?, &mut value);

        // Merge extend configs
        if let Some(ext) = &self.extend_configs {
            merge_yaml(ext, &mut value);
        }

        // Merge extend scripts
        if let Some(ext) = &self.extend_scripts {
            merge_scripts("Profile extend scripts", ext, &mut value)?;
        }

        // Merge global extend configs
        if let Some(ext) = &cfg.extend_configs {
            merge_yaml(ext, &mut value);
        }

        // Merge global extend scripts
        if let Some(ext) = &cfg.extend_scripts {
            merge_scripts("Global extend scripts", ext, &mut value)?;
        }

        // Write data
        fs::write(&cfg.mihomo_path, serde_yaml::to_string(&value)?)?;

        // Restart mihomo
        cfg.get_api().restart().await?;

        // Success
        Ok(())
    }
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ConfigType {
    Local,
    Remote,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigProxy {
    None,
    System,
    Mihomo,
}
impl Default for ConfigProxy {
    fn default() -> Self {
        Self::None
    }
}

fn merge_yaml(src: &Value, dst: &mut Value) {
    match (src, dst) {
        (Value::Mapping(src), dst @ &mut Value::Mapping(_)) => {
            let dst = dst.as_mapping_mut().unwrap();
            for (k, v) in src {
                if !dst.contains_key(k) {
                    dst.insert(k.clone(), v.clone());
                } else {
                    merge_yaml(v, &mut dst[&k]);
                }
            }
        }
        (src, dst) => *dst = src.clone(),
    }
}

fn merge_scripts<S1, S2>(name: S1, scripts: S2, dst: &mut Value) -> Result<()>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    // Create context
    let mut context = Context::default();

    // Set runtime limits
    let mut runtime_limits = RuntimeLimits::default();
    runtime_limits.set_loop_iteration_limit(1_048_576); // 1M
    runtime_limits.set_recursion_limit(1_048_576); // 1M
    runtime_limits.set_stack_size_limit(16_777_216); // 16M
    context.set_runtime_limits(runtime_limits);

    // Register WebAPI runtime
    boa_runtime::register(&mut context, RegisterOptions::new())
        .map_err(|err| anyhow!("{}", err))?;

    // Print header
    println!(
        "{}",
        console::style(format!(">>> JS Engine Output: {} <<<", name.as_ref()))
            .bold()
            .bright()
            .cyan()
    );

    // Evaluate input scripts
    let source = Source::from_bytes(scripts.as_ref().as_bytes());
    context.eval(source).map_err(|err| anyhow!("{}", err))?;

    // Prepare data
    let config = serde_json::to_string(dst)?;
    context
        .register_global_property(
            js_string!("__RAW_CONFIGS__"),
            js_string!(config),
            Attribute::all(),
        )
        .map_err(|err| anyhow!("{}", err))?;

    // Evaluate function
    let source = Source::from_bytes(r"JSON.stringify(main(JSON.parse(__RAW_CONFIGS__)))");
    let result = context
        .eval(source)
        .map_err(|err| anyhow!("{}", err))?
        .to_string(&mut context)
        .map_err(|err| anyhow!("{}", err))?
        .to_std_string_escaped();

    // Print footer
    println!(
        "{}",
        console::style(">>> End of Output <<<")
            .bold()
            .bright()
            .cyan()
    );

    // Success
    *dst = serde_json::from_str(&result)?;
    Ok(())
}
