use chrono::Utc;
use comfy_table::{Cell, Row};
use owo_colors::OwoColorize;

use crate::utils::IntoSizeStr;

use super::entity::*;

impl Model {
    pub fn row(&self) -> Row {
        if self.r#type.is_local() {
            vec![
                self.uuid.clone().into(),
                self.name.clone().into(),
                self.col_updated_at(),
            ]
        } else {
            vec![
                self.uuid.clone().into(),
                self.name.clone().into(),
                self.col_updated_at(),
                self.col_expired_at(),
                self.col_usage(),
            ]
        }
        .into()
    }

    fn col_updated_at(&self) -> Cell {
        match self.updated_at {
            Some(t) => match Utc::now().timestamp() - t.timestamp() {
                ..0 => "?".into(),
                0..60 => "now".into(), // Within 1 minute
                d @ ..3_600 => format!("~ {}min", d / 60).into(), // Within 1 hour
                d @ ..86_400 => format!("~ {}hr", d / 3600).into(), // Within 1 day
                d @ ..604_800 => format!("~ {}d", d / 86400).yellow().into(), // Within 1 week
                _ => t.format("%Y-%m-%d").red().into(),
            },
            None => "N/A".bright_black().into(),
        }
    }

    fn col_expired_at(&self) -> Cell {
        match self.expired_at {
            Some(t) => {
                if t > Utc::now() {
                    t.format("%Y-%m-%d").into()
                } else {
                    t.format("%Y-%m-%d").red().into()
                }
            }
            None => "N/A".bright_black().into(),
        }
    }

    fn col_usage(&self) -> Cell {
        if self.used_bytes.is_none() && self.total_bytes.is_none() {
            return "N/A".bright_black().into();
        }

        if let Some(used) = self.used_bytes
            && let Some(total) = self.total_bytes
        {
            let pct = match used as f64 / total as f64 * 100.0 {
                ..0.0 => "?".into(),
                x @ ..70.0 => format!("({x:.1}%)").green().to_string(),
                x @ ..90.0 => format!("({x:.1}%)").yellow().to_string(),
                x => format!("({x:.1}%)").red().to_string(),
            };
            return format!("{}/{} {}", used.into_size_str(), total.into_size_str(), pct).into();
        }

        if let Some(used) = self.used_bytes {
            return format!("{}/-", used.into_size_str()).into();
        }

        if let Some(total) = self.total_bytes {
            return format!("-/{}", total.into_size_str()).into();
        }

        unreachable!();
    }
}
