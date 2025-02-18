pub mod api;
pub mod file;
pub mod highlight;
pub mod path;
pub mod prompt;
pub mod result;

use anyhow::{anyhow, Result};
use rand::{rngs::OsRng, RngCore};
use serde::Deserialize;

pub fn get_size_str(x: usize) -> String {
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

pub fn gen_uuid() -> String {
    let mut rng = OsRng;
    let mut buf = vec![0u8; 4];
    rng.fill_bytes(&mut buf);
    hex::encode(buf)
}

pub fn extract_rules<S>(data: S) -> Result<String>
where
    S: AsRef<str>,
{
    // Extractor
    #[derive(Deserialize)]
    struct Extractor {
        rules: Option<Vec<String>>,
    }
    let value = serde_yaml::from_str::<Extractor>(data.as_ref())?;
    if value.rules.is_none() {
        return Ok(String::new());
    }
    let rules = value.rules.unwrap();

    // Build contents
    let mut contents = String::new();
    let idx_w = rules.len().to_string().len();
    for (idx, rule) in rules.iter().enumerate() {
        let (filter, rest) = rule
            .split_once(',')
            .ok_or(anyhow!("fail to extract filter from rule `{}`", rule))?;
        let (param, target) = rest.rsplit_once(',').unwrap_or(("", rest));

        contents.push_str(&format!(
            "[{:>w$}] {}({}) -> {}\n",
            idx + 1,
            console::style(filter).bold().bright().blue(),
            console::style(param).bright().yellow(),
            console::style(target).bold().bright().red(),
            w = idx_w
        ));
    }

    // Success
    Ok(contents)
}
