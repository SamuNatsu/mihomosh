use std::fs;

use anyhow::{Context, Result, bail};
use rand::{TryRngCore, rngs::OsRng};

use crate::{
    models::{
        meta::Meta,
        profile::{Profile, ProfileType},
    },
    println_secondary, println_success,
    utils::{dir, file, prompt},
};

const DEFAULT_CONFIG_TEMPLATE: &'static str = include_str!("../includes/default_profile.yaml");
const DEFAULT_EXTEND_CONFIG_TEMPLATE: &'static str =
    include_str!("../includes/default_extend_config.yaml");
const DEFAULT_EXTEND_SCRIPT_TEMPLATE: &'static str =
    include_str!("../includes/default_extend_script.js");

pub fn create(editor: String) -> Result<()> {
    let mut meta_map = Meta::get_instance().lock().unwrap();

    // Edit temporary file
    let contents = file::edit_temp_file(
        ".yaml",
        &editor,
        DEFAULT_CONFIG_TEMPLATE.replace("<CARGO_PKG_VERSION>", env!("CARGO_PKG_VERSION")),
    )
    .with_context(|| format!("Fail to edit temporary YAML file with editor `{}`", editor))?;

    let profile = serde_yml::from_str::<Profile>(&contents).context("Fail to parse profile")?;
    profile.verify().context("Fail to verify profile")?;

    // Confirm to create
    let msg = format!("Are you sure to create the new profile `{}`?", profile.name);
    let input = prompt::confirm(&msg).context("Fail to show confirm prompt")?;
    if !input {
        println_secondary!("Changes discarded");
        return Ok(());
    }

    // Update metadata
    let uuid = gen_uuid().context("Fail to generate UUID")?;
    meta_map.insert(
        uuid.clone(),
        Meta {
            name: profile.name.clone().trim().to_string(),
            is_remote: if let ProfileType::Local = profile.r#type {
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

    drop(meta_map);
    Meta::flush().context("Fail to flush metadata")?;

    // Write profile
    let path = Profile::get_path(&uuid);
    fs::write(&path, contents)
        .with_context(|| format!("Fail to write file `{}`", path.display()))?;

    // Success
    println_success!("New profile `{}` with UUID `{uuid}` added", profile.name);
    Ok(())
}

pub fn delete(uuid_or_name: String) -> Result<()> {
    let mut meta_map = Meta::get_instance().lock().unwrap();

    // Get metadata
    let uuid = {
        if meta_map.contains_key(&uuid_or_name) {
            &uuid_or_name
        } else {
            let entries = meta_map
                .iter()
                .filter(|(_, v)| v.name == uuid_or_name)
                .collect::<Vec<_>>();

            if entries.len() == 0 {
                bail!("Profile not found with UUID or name `{uuid_or_name}`");
            }
            if entries.len() > 1 {
                bail!(
                    "Multiple profiles found with name `{uuid_or_name}`, please use UUID instead"
                );
            }

            entries[0].0
        }
    }
    .clone();
    let meta = meta_map.get(&uuid).unwrap();
    let name = meta.name.clone();

    // Confirm to delete
    let msg = format!("Are your sure to delete profile `{name}` with UUID `{uuid}`",);
    let input = prompt::confirm(msg).context("Fail to show confirm prompt")?;
    if !input {
        println_secondary!("Changes discarded");
        return Ok(());
    }

    // Delete files
    let path = Profile::get_path(&uuid);
    fs::remove_file(&path).with_context(|| format!("Fail to remove file `{}`", path.display()))?;

    let path = Profile::get_data_path(&uuid);
    if path.is_file() {
        fs::remove_file(&path)
            .with_context(|| format!("Fail to remove file `{}`", path.display()))?;
    }

    let path = Profile::get_ext_conf_path(&uuid);
    if path.is_file() {
        fs::remove_file(&path)
            .with_context(|| format!("Fail to remove file `{}`", path.display()))?;
    }

    let path = Profile::get_ext_script_path(&uuid);
    if path.is_file() {
        fs::remove_file(&path)
            .with_context(|| format!("Fail to remove file `{}`", path.display()))?;
    }

    // Update metadata
    meta_map.remove(&uuid);

    drop(meta_map);
    Meta::flush().context("Fail to flush metadata")?;

    // Success
    println_success!("Profile `{name}` with UUID `{uuid}` deleted");
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
        if a.1.is_remote != b.1.is_remote {
            a.1.is_remote.cmp(&b.1.is_remote)
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

pub fn view_glob_ext_conf(viewer: String) -> Result<()> {
    // Check file
    let path = dir::get_data_dir().join("extend-config.yaml");
    if !path.is_file() {
        println_secondary!("No global extend configs");
        return Ok(());
    }

    // View file
    file::view_file(&viewer, &path).with_context(|| {
        format!(
            "Fail to view file `{}` with viewer `{}`",
            path.display(),
            viewer
        )
    })?;
    Ok(())
}

pub fn view_glob_ext_script(viewer: String) -> Result<()> {
    // Check file
    let path = dir::get_data_dir().join("extend-script.js");
    if !path.is_file() {
        println_secondary!("No global extend script");
        return Ok(());
    }

    // View file
    file::view_file(&viewer, &path).with_context(|| {
        format!(
            "Fail to view file `{}` with viewer `{}`",
            path.display(),
            viewer
        )
    })?;
    Ok(())
}

pub fn edit_glob_ext_conf(editor: String) -> Result<()> {
    // Check file
    let path = dir::get_data_dir().join("extend-config.yaml");
    if !path.is_file() {
        fs::write(&path, DEFAULT_EXTEND_CONFIG_TEMPLATE)
            .with_context(|| format!("Fail to write file `{}`", path.display()))?;
    }

    // Edit file
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Fail to read file `{}`", path.display()))?;
    let contents = file::edit_temp_file(".yaml", &editor, &contents)
        .with_context(|| format!("Fail to edit temporary YAML file with editor `{}`", editor))?;

    // Confirm to save
    let input = prompt::confirm("Are you sure to save the changes?")
        .context("Fail to show confirm prompt")?;
    if !input {
        println_secondary!("Changes discarded");
        return Ok(());
    }

    // Update configs
    fs::write(&path, contents)
        .with_context(|| format!("Fail to write file `{}`", path.display()))?;
    println_success!("Global extend config saved");
    Ok(())
}

pub fn edit_glob_ext_script(editor: String) -> Result<()> {
    // Check file
    let path = dir::get_data_dir().join("extend-script.js");
    if !path.is_file() {
        fs::write(&path, DEFAULT_EXTEND_SCRIPT_TEMPLATE)
            .with_context(|| format!("Fail to write file `{}`", path.display()))?;
    }

    // Edit file
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Fail to read file `{}`", path.display()))?;
    let contents = file::edit_temp_file(".js", &editor, &contents)
        .with_context(|| format!("Fail to edit temporary YAML file with editor `{}`", editor))?;

    // Confirm to save
    let input = prompt::confirm("Are you sure to save the changes?")
        .context("Fail to show confirm prompt")?;
    if !input {
        println_secondary!("Changes discarded");
        return Ok(());
    }

    // Update script
    fs::write(&path, contents)
        .with_context(|| format!("Fail to write file `{}`", path.display()))?;
    println_success!("Global extend scripts saved");
    Ok(())
}

fn gen_uuid() -> Result<String> {
    let mut rng = OsRng;
    let mut buf = vec![0u8; 4];

    rng.try_fill_bytes(&mut buf)
        .context("Fail to fill random bytes")?;
    Ok(hex::encode(buf))
}
