use std::fs;

use anyhow::Result;

use crate::{
    models::{
        meta::Meta,
        profile::{Profile, ProfileType},
    },
    println_secondary, println_success,
    utils::{self, file},
};

const DEFAULT_CONFIG_TEMPLATE: &'static str = include_str!("../includes/default_profile.yaml");

pub fn create(editor: String) -> Result<()> {
    let mut meta = Meta::get_instance().lock().unwrap();

    // Edit temporary file
    let contents = file::edit_temp_file(
        ".yaml",
        editor,
        DEFAULT_CONFIG_TEMPLATE.replace("<CARGO_PKG_VERSION>", env!("CARGO_PKG_VERSION")),
    )?;
    let profile = serde_yml::from_str::<Profile>(&contents)?;
    profile.verify()?;

    // Confirm to create
    let prompt = format!(
        "Are you sure to create the new profile `{}`? (y/N) ",
        profile.name
    );
    let input = utils::prompt(&prompt)?;
    if input.to_lowercase() != "y" {
        println_secondary!("Changes discarded");
        return Ok(());
    }

    // Update metadata
    let uuid = utils::gen_uuid()?;
    meta.insert(
        uuid.clone(),
        Meta {
            name: profile.name.clone().trim().to_string(),
            remote: if let ProfileType::Local = profile.r#type {
                false
            } else {
                true
            },
            updated_at: None,
            expired_at: None,
            used_bytes: None,
            total_bytes: None,
        },
    );
    Meta::flush()?;

    // Update config file
    let path = Profile::get_path(&uuid);
    fs::write(&path, contents)?;

    // Success
    println_success!("New profile `{}` with UUID `{}` added", profile.name, uuid);
    Ok(())
}

pub fn list() -> Result<()> {
    let meta = Meta::get_instance().lock().unwrap();

    // If no profile
    if meta.len() == 0 {
        println_secondary!("No profile");
        return Ok(());
    }

    // Get profile list
    let mut kv = meta.iter().collect::<Vec<_>>();
    kv.sort_by(|a, b| {
        if a.1.remote != b.1.remote {
            a.1.remote.cmp(&b.1.remote)
        } else if a.1.name != b.1.name {
            a.1.name.cmp(&b.1.name)
        } else {
            a.0.cmp(b.0)
        }
    });

    // Print list
    println!(
        "{:^8}    {:16}    {:^6}    {:^10}    {:^10}    {}",
        console::style("UUID").bold().bright().blue(),
        console::style("Name").bold().bright().blue(),
        console::style("Remote").bold().bright().blue(),
        console::style("Updated At").bold().bright().blue(),
        console::style("Expired At").bold().bright().blue(),
        console::style("Usage").bold().bright().blue()
    );
    for (k, v) in kv {
        println!(
            "{:^8}    {:16}    {:^6}    {:^10}    {:^10}    {}",
            k,
            v.get_styled_name(),
            v.get_styled_remote(),
            v.get_styled_duration(),
            v.get_styled_expired_at(),
            v.get_styled_usage(),
        );
    }

    // Success
    Ok(())
}
