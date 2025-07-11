use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use console::StyledObject;
use serde::{Deserialize, Serialize};
use unicode_width::UnicodeWidthStr;

use crate::{style_fmt, utils::dir};

#[derive(Deserialize, Serialize)]
pub struct Meta {
    pub name: String,
    pub is_remote: bool,
    pub updated_at: Option<i64>,
    pub expired_at: Option<i64>,
    pub used_bytes: Option<usize>,
    pub total_bytes: Option<usize>,
}

impl Meta {
    pub fn get_path() -> &'static PathBuf {
        static INSTANCE: OnceLock<PathBuf> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let path = dir::get_data_dir().join("meta.json");
            if !path.is_file() {
                fs::write(&path, "{}")
                    .with_context(|| format!("Fail to write file `{}`", path.display()))
                    .unwrap();
            }

            path
        })
    }

    pub fn get_instance() -> &'static Mutex<HashMap<String, Self>> {
        static INSTANCE: OnceLock<Mutex<HashMap<String, Meta>>> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let path = Self::get_path();
            let file = File::open(path)
                .with_context(|| format!("Fail to open file `{}`", path.display()))
                .unwrap();

            serde_json::from_reader(&file)
                .with_context(|| format!("Fail to parse meta file `{}`", path.display()))
                .unwrap()
        })
    }

    pub fn flush() -> Result<()> {
        let meta = Self::get_instance().lock().unwrap();
        let path = Self::get_path();
        let file = File::create(path)
            .with_context(|| format!("Fail to create file `{}`", path.display()))?;
        serde_json::to_writer(&file, &*meta)
            .with_context(|| format!("Fail to serialize and write file `{}`", path.display()))?;

        Ok(())
    }

    pub fn get_styled_name(&self) -> String {
        if UnicodeWidthStr::width_cjk(self.name.as_str()) > 16 {
            let chars = self.name.chars().collect::<Vec<_>>();
            let mut ed = 0;
            loop {
                let tmp = chars[..ed].iter().collect::<String>();
                if UnicodeWidthStr::width_cjk(tmp.as_str()) > 13 {
                    ed -= 1;
                    break;
                }
                ed += 1;
            }

            let mut tmp = chars[..ed].iter().collect::<String>();
            tmp.push_str("...");
            tmp
        } else {
            self.name.clone()
        }
    }

    pub fn get_styled_remote(&self) -> StyledObject<&'static str> {
        if self.is_remote {
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
                console::style("now".to_owned())
            } else if delta < 3600 {
                style_fmt!("~ {}min", delta / 60)
            } else if delta < 86400 {
                style_fmt!("~ {}hr", delta / 3600)
            } else if delta < 604800 {
                style_fmt!("~ {}d", delta / 86400).yellow()
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
            console::style("N/A".to_owned()).bright().black()
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
            console::style("N/A".to_owned()).bright().black()
        }
    }

    pub fn get_styled_usage(&self) -> StyledObject<String> {
        if self.used_bytes.is_none() && self.total_bytes.is_none() {
            return console::style("N/A".to_owned()).bright().black();
        }

        if self.used_bytes.is_some() && self.total_bytes.is_some() {
            let percent =
                self.used_bytes.unwrap() as f64 / self.total_bytes.unwrap() as f64 * 100.0;
            let percent = if percent < 70.0 {
                style_fmt!("({:.1}%)", percent).green()
            } else if percent < 90.0 {
                style_fmt!("({:.1}%)", percent).yellow()
            } else {
                style_fmt!("({:.1}%)", percent).red()
            };

            return style_fmt!(
                "{}/{} {}",
                get_size_str(self.used_bytes.unwrap()),
                get_size_str(self.total_bytes.unwrap()),
                percent
            );
        }

        if let Some(used) = self.used_bytes {
            style_fmt!("{}/-", get_size_str(used))
        } else {
            style_fmt!("-/{}", get_size_str(self.total_bytes.unwrap()))
        }
    }
}

fn get_size_str(x: usize) -> String {
    if x < 1_024 {
        return format!("{}B", x);
    } else if x < 1_048_576 {
        return format!("{:.1}KB", x as f64 / 1_024f64);
    } else if x < 1_073_741_824 {
        return format!("{:.1}MB", x as f64 / 1_048_576f64);
    } else {
        return format!("{:.1}GB", x as f64 / 1_073_741_824f64);
    }
}
