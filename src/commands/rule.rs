use anyhow::{Context, Result};

use crate::{models::config::Config, println_secondary, str_danger, str_primary, str_warn};

pub async fn print_rule() -> Result<()> {
    let rules = Config::get_instance()
        .get_api()
        .get_rules()
        .await
        .context("Fail to get rules")?;

    if rules.is_empty() {
        println_secondary!("No rule");
        return Ok(());
    }

    let cnt_len = rules.len().to_string().len();
    for (idx, rule) in rules.iter().enumerate() {
        if rule.payload.is_empty() {
            println!(
                "[{:>w$}] {} -> {}",
                idx + 1,
                str_primary!("{}", rule.r#type),
                str_danger!("{}", rule.proxy),
                w = cnt_len
            );
        } else {
            println!(
                "[{:>w$}] {}({}) -> {}",
                idx + 1,
                str_primary!("{}", rule.r#type),
                str_warn!("{}", rule.payload),
                str_danger!("{}", rule.proxy),
                w = cnt_len
            );
        }
    }

    Ok(())
}
