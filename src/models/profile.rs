use std::{
    fs::{self, File},
    path::PathBuf,
    time::Duration,
};

use anyhow::{Context, Result, anyhow, bail};
use boa_engine::{
    Context as BoaContext, Source, js_string, property::Attribute, vm::RuntimeLimits,
};
use boa_runtime::RegisterOptions;
use reqwest::{ClientBuilder, Proxy};
use serde::Deserialize;
use serde_yml::{Mapping, Value};
use url::Url;

use crate::{models::config::Config, println_help, println_warn, utils::dir};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    pub name: String,
    pub r#type: ProfileType,
    pub url: Option<Url>,
    pub user_agent: Option<String>,
    pub proxy: Option<ProfileProxy>,
    pub allow_invalid_certs: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProfileType {
    Remote,
    Local,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProfileProxy {
    None,
    System,
    Mihomo,
}

impl Profile {
    pub fn get_path<S: AsRef<str>>(uuid: S) -> PathBuf {
        dir::get_profile_dir().join(format!("{}.yaml", uuid.as_ref()))
    }

    pub fn get_data_path<S: AsRef<str>>(uuid: S) -> PathBuf {
        dir::get_profile_dir().join(format!("{}.data.yaml", uuid.as_ref()))
    }

    pub fn get_ext_conf_path<S: AsRef<str>>(uuid: S) -> PathBuf {
        dir::get_profile_dir().join(format!("{}.ext.yaml", uuid.as_ref()))
    }

    pub fn get_ext_script_path<S: AsRef<str>>(uuid: S) -> PathBuf {
        dir::get_profile_dir().join(format!("{}.ext.js", uuid.as_ref()))
    }

    pub fn load<S: AsRef<str>>(uuid: S) -> Result<Self> {
        let path = Self::get_path(uuid);
        let file =
            File::open(&path).with_context(|| format!("Fail to open file `{}`", path.display()))?;
        let ret: Self = serde_yml::from_reader(&file)
            .with_context(|| format!("Fail to parse file `{}`", path.display()))?;
        ret.verify()
            .with_context(|| format!("Fail to verify file `{}`", path.display()))?;

        Ok(ret)
    }

    pub fn verify(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            bail!("name cannot be empty");
        }

        if let ProfileType::Remote = self.r#type {
            if self.url.is_none() {
                bail!("`url` cannot be empty");
            }

            if self.user_agent.as_deref().unwrap_or("").trim().is_empty() {
                bail!("`user-agent` cannot be empty");
            }

            if self.proxy.is_none() {
                bail!("`proxy` is needed");
            }

            if self.allow_invalid_certs.is_none() {
                bail!("`allow-invalid-certs` is needed")
            }
        }

        Ok(())
    }

    pub async fn activate<S: AsRef<str>>(&self, uuid: S) -> Result<()> {
        let cfg = Config::get_instance();

        // Load data
        let path = Self::get_data_path(&uuid);
        let mut value = if path.is_file() {
            let file = File::open(&path)
                .with_context(|| format!("Fail to open file `{}`", path.display()))?;
            serde_yml::from_reader(&file)
                .with_context(|| format!("Fail to parse file `{}`", path.display()))?
        } else {
            Value::Mapping(Mapping::new())
        };
        if !value.is_mapping() {
            println_warn!(
                "Profile data with UUID `{}` is not an object, force to use empty object",
                uuid.as_ref()
            );
            value = Value::Mapping(Mapping::new());
        }

        // Merge mihomosh configs
        let tmp = format!(
            "mode: {}\nallow-lan: {}\nipv6: {}\nunified-delay: {}\nport: {}\nsocks-port: {}\nmixed-port: {}\nlog-level: {}\nredir-port: 0\ntproxy-port: 0\nsecret: {}\n",
            cfg.mode.as_str(),
            cfg.allow_lan,
            cfg.allow_ipv6,
            cfg.unified_delay,
            cfg.port.unwrap_or_default(),
            cfg.socks_port.unwrap_or_default(),
            cfg.mixed_port.unwrap_or_default(),
            cfg.log_level.as_str(),
            cfg.mihomo_secret.as_deref().unwrap_or_default()
        );
        let tmp = serde_yml::from_str::<Value>(&tmp).context("Fail to parse prepared configs")?;
        merge_yaml(&tmp, &mut value);

        // Merge extend configs
        let path = Self::get_ext_conf_path(&uuid);
        if path.is_file() {
            let file = File::open(&path)
                .with_context(|| format!("Fail to open file `{}`", path.display()))?;
            let tmp: Value = serde_yml::from_reader(&file)
                .with_context(|| format!("Fail to parse file `{}`", path.display()))?;

            if tmp.is_mapping() {
                merge_yaml(&tmp, &mut value);
            } else {
                println_warn!(
                    "Profile extend configs with UUID `{}` is not an object, skipped",
                    uuid.as_ref()
                );
            }
        }

        // Merge extend script
        let path = Self::get_ext_script_path(&uuid);
        if path.is_file() {
            let contents = fs::read_to_string(&path)
                .with_context(|| format!("Fail to read file `{}`", path.display()))?;
            merge_scripts("Profile extend script", &contents, &mut value).with_context(|| {
                format!("Fail to merge extend script from file `{}`", path.display())
            })?;
        }

        // Merge global extend configs
        let path = dir::get_data_dir().join("extend.yaml");
        if path.is_file() {
            let file = File::open(&path)
                .with_context(|| format!("Fail to open file `{}`", path.display()))?;
            let tmp: Value = serde_yml::from_reader(&file)
                .with_context(|| format!("Fail to parse file `{}`", path.display()))?;

            if tmp.is_mapping() {
                merge_yaml(&tmp, &mut value);
            } else {
                println_warn!("Global extend configs is not an object, skipped");
            }
        }

        // Merge global extend scripts
        let path = dir::get_data_dir().join("extend.js");
        if path.is_file() {
            let contents = fs::read_to_string(&path)
                .with_context(|| format!("Fail to read file `{}`", path.display()))?;
            merge_scripts("Global extend script", &contents, &mut value).with_context(|| {
                format!("Fail to merge extend script from file `{}`", path.display())
            })?;
        }

        // Write data
        let data = serde_yml::to_string(&value).context("Fail to serialize configs")?;
        fs::write(&cfg.mihomo_path, &data)
            .with_context(|| format!("Fail to write file `{}`", &cfg.mihomo_path.display()))?;

        // Restart mihomo
        cfg.get_api()
            .restart()
            .await
            .context("Fail to restart Mihomo")?;

        // Success
        Ok(())
    }

    pub async fn update<S: AsRef<str>>(&self, uuid: S) -> Result<SubUserInfo> {
        // Check local
        if matches!(self.r#type, ProfileType::Local) {
            bail!("Not allow to update a local profile");
        }

        // Fetch data
        let cfg = Config::get_instance();
        let mut builder = ClientBuilder::new()
            .danger_accept_invalid_certs(self.allow_invalid_certs.unwrap())
            .timeout(Duration::from_secs(30));
        match self.proxy.as_ref().unwrap() {
            ProfileProxy::None => builder = builder.no_proxy(),
            ProfileProxy::System => (),
            ProfileProxy::Mihomo => {
                let proxy = if let Some(port) = cfg.port {
                    Proxy::all(format!("http://127.0.0.1:{port}"))
                        .with_context(|| format!("Fail to set proxy `http://127.0.0.1:{port}`"))?
                } else if let Some(port) = cfg.socks_port {
                    Proxy::all(format!("socks://127.0.0.1:{port}"))
                        .with_context(|| format!("Fail to set proxy `socks://127.0.0.1:{port}`"))?
                } else if let Some(port) = cfg.mixed_port {
                    Proxy::all(format!("http://127.0.0.1:{port}"))
                        .with_context(|| format!("Fail to set proxy `http://127.0.0.1:{port}`"))?
                } else {
                    bail!(
                        "Mihomo proxy is not available, please enable at lease one listening port"
                    );
                };
                builder = builder.proxy(proxy);
            }
        };
        let resp = builder
            .user_agent(self.user_agent.as_ref().unwrap())
            .build()
            .context("Fail to create reqwest client")?
            .get(self.url.as_ref().unwrap().clone())
            .send()
            .await
            .with_context(|| format!("Fail to send `GET {}`", self.url.as_ref().unwrap()))?;

        // Parse data
        let mut ret = SubUserInfo::default();
        if let Some(h) = resp.headers().get("Subscription-Userinfo") {
            let segments = h
                .to_str()
                .with_context(|| format!("Fail to parse header `{h:?}`"))?
                .split(';');
            for seg in segments {
                if let Some((k, v)) = seg.trim().split_once('=') {
                    let v = match v.parse::<usize>() {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                    match k {
                        "upload" => {
                            ret.used = if let Some(tmp) = ret.used {
                                Some(tmp + v)
                            } else {
                                Some(v)
                            }
                        }
                        "download" => {
                            ret.used = if let Some(tmp) = ret.used {
                                Some(tmp + v)
                            } else {
                                Some(v)
                            }
                        }
                        "total" => ret.total = Some(v),
                        "expire" => ret.expired_at = Some(v as i64),
                        _ => (),
                    }
                }
            }
        }

        let body = resp
            .text()
            .await
            .with_context(|| format!("Fail to get `GET {}`", self.url.as_ref().unwrap()))?;

        // Save file
        let path = Self::get_data_path(uuid);
        fs::write(&path, &body)
            .with_context(|| format!("Fail to write file `{}`", path.display()))?;

        // Success
        Ok(ret)
    }
}

#[derive(Default)]
pub struct SubUserInfo {
    pub used: Option<usize>,
    pub total: Option<usize>,
    pub expired_at: Option<i64>,
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
    let mut context = BoaContext::default();

    // Set runtime limits
    let mut runtime_limits = RuntimeLimits::default();
    runtime_limits.set_loop_iteration_limit(1_048_576); // 1M
    runtime_limits.set_recursion_limit(1_048_576); // 1M
    runtime_limits.set_stack_size_limit(16_777_216); // 16M
    context.set_runtime_limits(runtime_limits);

    // Register WebAPI runtime
    boa_runtime::register(&mut context, RegisterOptions::new())
        .map_err(|err| anyhow!("{err}"))
        .context("Fail to register WebAPI runtime")?;

    // Print header
    println_help!(">>> JS Engine Output: {} <<<", name.as_ref());

    // Evaluate input scripts
    let source = Source::from_bytes(scripts.as_ref().as_bytes());
    context
        .eval(source)
        .map_err(|err| anyhow!("{err}"))
        .context("Fail to evaluate script")?;

    // Prepare data
    let config = serde_json::to_string(dst).context("Fail to stringify raw configs")?;
    context
        .register_global_property(
            js_string!("__RAW_CONFIGS__"),
            js_string!(config),
            Attribute::all(),
        )
        .map_err(|err| anyhow!("{err}"))
        .context("Fail to register raw configs")?;

    // Evaluate function
    let source = Source::from_bytes(r"JSON.stringify(main(JSON.parse(__RAW_CONFIGS__)))");
    let result = context
        .eval(source)
        .map_err(|err| anyhow!("{err}"))
        .context("Fail to evaluate script")?
        .to_string(&mut context)
        .map_err(|err| anyhow!("{err}"))
        .context("Fail to parse script output")?
        .to_std_string_escaped();

    // Print footer
    println_help!(">>> End of Output <<<");

    // Success
    *dst = serde_json::from_str(&result)?;
    Ok(())
}
