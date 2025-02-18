use std::fs;

use anyhow::Result;
use chrono::{TimeZone, Utc};
use console::StyledObject;
use serde::{Deserialize, Serialize};

use crate::utils::{self, path};

use super::Config;

/// Metadata
#[derive(Clone, Deserialize, Serialize)]
pub struct Meta {
    #[serde(skip)]
    pub uuid: String,

    pub name: String,
    pub remote: bool,
    pub updated_at: Option<i64>,
    pub expired_at: Option<i64>,
    pub used_bytes: Option<usize>,
    pub total_bytes: Option<usize>,
}
impl Meta {
    pub fn try_get_conf(&self) -> Result<Config> {
        let path = path::get_profile_conf_dir().join(format!("{}.yaml", self.uuid));
        let contents = fs::read_to_string(&path)?;
        let mut value = serde_yaml::from_str::<Config>(&contents)?;
        value.uuid = self.uuid.clone();

        Ok(value)
    }

    pub fn get_styled_name(&self) -> String {
        if self.name.len() > 16 {
            let mut tmp = self.name[..13].to_string();
            tmp.push_str("...");
            tmp
        } else {
            self.name.clone()
        }
    }

    pub fn get_styled_remote(&self) -> StyledObject<&str> {
        if self.remote {
            console::style("Y").green()
        } else {
            console::style("N").red()
        }
    }

    pub fn get_styled_duration(&self) -> StyledObject<String> {
        if let Some(ts) = self.updated_at {
            let now_ts = Utc::now().timestamp();
            let delta = now_ts - ts;

            if delta < 60 {
                console::style("now".into())
            } else if delta < 3600 {
                console::style(format!("~ {}min", delta / 60))
            } else if delta < 86400 {
                console::style(format!("~ {}hr", delta / 3600))
            } else if delta < 604800 {
                console::style(format!("~ {}d", delta / 86400)).yellow()
            } else {
                console::style(
                    Utc.timestamp_opt(ts, 0)
                        .unwrap()
                        .format("%Y-%m-%d")
                        .to_string(),
                )
                .red()
            }
        } else {
            console::style("N/A".into()).bright().black()
        }
    }

    pub fn get_styled_expired_at(&self) -> StyledObject<String> {
        if let Some(ts) = self.expired_at {
            console::style(
                Utc.timestamp_opt(ts, 0)
                    .unwrap()
                    .format("%Y-%m-%d")
                    .to_string(),
            )
        } else {
            console::style("N/A".into()).bright().black()
        }
    }

    pub fn get_styled_usage(&self) -> StyledObject<String> {
        if self.used_bytes.is_none() && self.total_bytes.is_none() {
            return console::style("N/A".into()).bright().black();
        }

        if self.used_bytes.is_some() && self.total_bytes.is_some() {
            let percent =
                self.used_bytes.unwrap() as f64 / self.total_bytes.unwrap() as f64 * 100.0;
            let percent = if percent < 70.0 {
                console::style(format!("({:.1}%)", percent)).green()
            } else if percent < 90.0 {
                console::style(format!("({:.1}%)", percent)).yellow()
            } else {
                console::style(format!("({:.1}%)", percent)).red()
            };

            return console::style(format!(
                "{}/{} {}",
                utils::get_size_str(self.used_bytes.unwrap()),
                utils::get_size_str(self.total_bytes.unwrap()),
                percent
            ));
        }

        if let Some(used) = self.used_bytes {
            console::style(format!("{}/-", utils::get_size_str(used)))
        } else {
            console::style(format!(
                "-/{}",
                utils::get_size_str(self.total_bytes.unwrap())
            ))
        }
    }
}
