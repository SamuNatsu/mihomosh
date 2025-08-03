use anyhow::{Context, Result, bail};

use crate::{
    arguments::proxy::ProxyArgs, models::config::Config, println_help, println_secondary,
    println_success, str_danger, str_primary, str_success, str_warn, utils::prompt,
};

pub async fn handle_proxy(args: ProxyArgs) -> Result<()> {
    match args {
        ProxyArgs::View => view().await?,
        ProxyArgs::Update => update().await?,
        ProxyArgs::Test { url, timeout } => test(url, timeout).await?,
    }
    Ok(())
}

async fn view() -> Result<()> {
    let mut groups = Config::get_instance()
        .get_api()
        .get_groups()
        .await
        .context("Fail to get groups")?;
    groups.sort_by_key(|group| group.name.clone());

    // If empty
    if groups.is_empty() {
        println_secondary!("No proxy");
        return Ok(());
    }

    // Print list
    for group in groups {
        println!(
            "{}({}) -> {}",
            str_primary!("{}", group.name),
            str_warn!("{}", group.r#type),
            str_danger!("{}", group.now)
        );
    }

    // Success
    Ok(())
}

async fn update() -> Result<()> {
    let mut groups = Config::get_instance()
        .get_api()
        .get_groups()
        .await
        .context("Fail to get groups")?;
    groups.sort_by_key(|group| group.name.clone());

    // If empty
    if groups.is_empty() {
        println_secondary!("No proxy");
        return Ok(());
    }

    // Print candidate list
    let cnt_len = groups.len().to_string().len();
    for (idx, group) in groups.iter().enumerate() {
        println!(
            "[{:>w$}] {}({}) -> {}",
            idx + 1,
            str_primary!("{}", group.name),
            str_warn!("{}", group.r#type),
            str_danger!("{}", group.now),
            w = cnt_len
        );
    }

    // Get group selection
    let group_idx = prompt::read_int(format!(
        "Please select group for updating: (1~{}) ",
        groups.len()
    ))
    .context("Fail to read integer")?;
    if group_idx < 1 || group_idx as usize > groups.len() {
        bail!(
            "Invalid selection, integer must between 1 and {}",
            groups.len()
        );
    }

    // Print proxy candidate
    let mut all = groups.get(group_idx as usize - 1).unwrap().all.clone();
    all.sort();

    let cnt_len = all.len().to_string().len();
    for (idx, proxy) in all.iter().enumerate() {
        println!(
            "[{:>w$}] {}",
            idx + 1,
            str_primary!("{}", proxy),
            w = cnt_len
        );
    }

    // Get proxy selection
    let proxy_idx = prompt::read_int(format!(
        "Please select proxy for updating: (1~{}) ",
        all.len()
    ))
    .context("Fail to read integer")?;
    if proxy_idx < 1 || proxy_idx as usize > all.len() {
        bail!(
            "Invalid selection, integer must between 1 and {}",
            all.len()
        );
    }

    // Updating
    let name = &groups.get(group_idx as usize - 1).unwrap().name;
    let proxy = all.get(proxy_idx as usize - 1).unwrap();
    Config::get_instance()
        .get_api()
        .update_proxy(name, proxy)
        .await
        .context("Fail to update proxy")?;

    // Success
    println_success!("Group `{name}` updated with proxy `{proxy}`");
    Ok(())
}

async fn test(url: String, timeout: u64) -> Result<()> {
    let mut groups = Config::get_instance()
        .get_api()
        .get_groups()
        .await
        .context("Fail to get groups")?;
    groups.sort_by_key(|group| group.name.clone());

    // If empty
    if groups.is_empty() {
        println_secondary!("No proxy");
        return Ok(());
    }

    // Print candidate list
    let cnt_len = groups.len().to_string().len();
    for (idx, group) in groups.iter().enumerate() {
        println!(
            "[{:>w$}] {}({}) -> {}",
            idx + 1,
            str_primary!("{}", group.name),
            str_warn!("{}", group.r#type),
            str_danger!("{}", group.now),
            w = cnt_len
        );
    }

    // Get selection
    let input = prompt::read_int(format!(
        "Please select proxy for testing: (1~{}) ",
        groups.len()
    ))
    .context("Fail to read integer")?;
    if input < 1 || input as usize > groups.len() {
        bail!(
            "Invalid selection, integer must between 1 and {}",
            groups.len()
        );
    }

    // Testing
    let name = &groups.get(input as usize - 1).unwrap().name;
    println_help!("Testing proxy `{name}` with URL `{url}` and timeout {timeout}ms...\n");

    let res = Config::get_instance()
        .get_api()
        .test_group(name, url, timeout)
        .await
        .context("Fail to test proxy")?;

    // Print result
    let all = &groups.get(input as usize - 1).unwrap().all;
    for name in all {
        println!(
            "{}: {}",
            str_primary!("{name}"),
            match res.get(name) {
                Some(v) =>
                    if v < &400 {
                        str_success!("{v}ms")
                    } else {
                        str_warn!("{v}ms")
                    },
                None => str_danger!("Timeout"),
            }
        );
    }

    // Success
    Ok(())
}
