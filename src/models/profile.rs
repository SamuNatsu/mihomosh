use std::{
    fs::{self, File},
    path::PathBuf,
    time::Duration,
};

use anyhow::{Context, Result, bail};
use reqwest::{ClientBuilder, Proxy};
use serde::Deserialize;
use url::Url;

use crate::{models::config::Config, utils::dir};

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
        if self.name.trim().len() == 0 {
            bail!("name cannot be empty");
        }

        if let ProfileType::Remote = self.r#type {
            if self.user_agent.as_deref().unwrap_or("").trim().len() == 0 {
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

    pub async fn update<S: AsRef<str>>(&self, uuid: S) -> Result<SubUserInfo> {
        // Check local
        if let ProfileType::Local = self.r#type {
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
            .get(self.url.as_ref().unwrap().to_string())
            .send()
            .await
            .with_context(|| format!("Fail to send `GET {}`", self.url.as_ref().unwrap()))?;

        // Parse data
        let mut ret = SubUserInfo::default();
        if let Some(h) = resp.headers().get("Subscription-Userinfo") {
            let segments = h
                .to_str()
                .with_context(|| format!("Fail to parse header `{:?}`", h))?
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
