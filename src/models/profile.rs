use std::path::PathBuf;

use anyhow::{Result, bail};
use serde::Deserialize;

use crate::utils::dir;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    pub name: String,
    pub r#type: ProfileType,
    pub url: Option<String>,
    pub user_agent: Option<String>,
    pub proxy: Option<ProfileProxy>,
    pub allow_invalid_caerts: Option<bool>,
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
        dir::get_profile_dir().join(format!("{}.js", uuid.as_ref()))
    }

    pub fn verify(&self) -> Result<()> {
        if self.name.trim().len() == 0 {
            bail!("name cannot be empty");
        }

        if let ProfileType::Remote = self.r#type {
            if self.url.as_deref().unwrap_or("").trim().len() == 0 {
                bail!("`url` cannot be empty");
            }

            if self.user_agent.as_deref().unwrap_or("").trim().len() == 0 {
                bail!("`user-agent` cannot be empty");
            }

            if self.proxy.is_none() {
                bail!("`proxy` is needed");
            }

            if self.allow_invalid_caerts.is_none() {
                bail!("`allow-invalid-certs` is needed")
            }
        }

        Ok(())
    }
}
