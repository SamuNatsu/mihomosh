use std::fs;

use anyhow::{Context, Result};
use chrono::Utc;
use rand::{TryRngCore, rngs::OsRng};
use tokio::task::JoinSet;

use crate::{
    includes::DEFAULT_PROFILE_TEMPLATE,
    models::{
        meta::Meta,
        profile::{Profile, ProfileType, SubUserInfo},
    },
    println_danger, println_primary, println_secondary, println_success,
    utils::{file, prompt},
};

pub async fn update(uuid_or_name: Option<String>) -> Result<()> {
    match uuid_or_name {
        Some(uuid_or_name) => {
            let uuid = Meta::find_uuid_or_name(&uuid_or_name)
                .with_context(|| format!("Fail to find UUID or name `{uuid_or_name}`"))?;
            let profile = Profile::load(&uuid).with_context(|| {
                format!("Fail to load profile with UUID or name `{uuid_or_name}`")
            })?;

            // Fetch data
            println_primary!("Updating profile `{}` with UUID `{uuid}`...", profile.name);
            let info = profile.update(&uuid).await.with_context(|| {
                format!(
                    "Fail to update profile `{}`, with UUID `{uuid}`",
                    profile.name
                )
            })?;

            // Update metadata
            let mut meta_map = Meta::get_instance().lock().unwrap();
            let meta = meta_map.get_mut(&uuid).unwrap();
            meta.used_bytes = info.used;
            meta.total_bytes = info.total;
            meta.expired_at = info.expired_at;
            meta.updated_at = Some(Utc::now().timestamp());

            drop(meta_map);
            Meta::flush().context("Fail to flush metadata")?;

            // Success
            println_success!("Profile `{}` with UUID `{uuid}` updated", profile.name);
            Ok(())
        }
        None => {
            let uuid = Meta::get_instance()
                .lock()
                .unwrap()
                .iter()
                .filter(|(_, v)| v.is_remote)
                .map(|(k, _)| k.clone())
                .collect::<Vec<_>>();

            // Create tasks
            let mut set = JoinSet::new();
            for uuid in uuid {
                set.spawn(async {
                    let uuid = uuid;
                    let res: Result<SubUserInfo> = async {
                        let profile = Profile::load(&uuid)
                            .with_context(|| format!("Fail to load profile with UUID `{uuid}`"))?;

                        println_primary!(
                            "Updating profile `{}` with UUID `{uuid}`...",
                            profile.name
                        );
                        let info = profile.update(&uuid).await.with_context(|| {
                            format!(
                                "Fail to update profile `{}`, with UUID `{uuid}`",
                                profile.name
                            )
                        })?;

                        println_success!("Profile `{}` with UUID `{uuid}` updated", profile.name);
                        Ok(info)
                    }
                    .await;

                    (uuid, res)
                });
            }

            // Solve tasks
            let res = set.join_all().await;
            let mut meta_map = Meta::get_instance().lock().unwrap();
            for (uuid, res) in res {
                match res {
                    Ok(info) => {
                        let meta = meta_map.get_mut(&uuid).unwrap();
                        meta.used_bytes = info.used;
                        meta.total_bytes = info.total;
                        meta.expired_at = info.expired_at;
                        meta.updated_at = Some(Utc::now().timestamp());
                    }
                    Err(err) => {
                        println_danger!("{err:?}");
                    }
                }
            }
            drop(meta_map);
            Meta::flush().context("Fail to flush metadata")?;
            Ok(())
        }
    }
}

pub fn create(editor: String) -> Result<()> {
    let mut meta_map = Meta::get_instance().lock().unwrap();

    // Edit temporary file
    let contents = file::edit_temp_file(
        ".yaml",
        &editor,
        DEFAULT_PROFILE_TEMPLATE.replace("<CARGO_PKG_VERSION>", env!("CARGO_PKG_VERSION")),
    )
    .with_context(|| format!("Fail to edit temporary YAML file with editor `{editor}`"))?;

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
            is_remote: matches!(profile.r#type, ProfileType::Remote),
            ..Default::default()
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
    // Get metadata
    let uuid = Meta::find_uuid_or_name(&uuid_or_name)
        .with_context(|| format!("Fail to find UUID or name `{uuid_or_name}`"))?;
    let mut meta_map = Meta::get_instance().lock().unwrap();
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
    if meta.is_empty() {
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

fn gen_uuid() -> Result<String> {
    let mut rng = OsRng;
    let mut buf = vec![0u8; 4];

    rng.try_fill_bytes(&mut buf)
        .context("Fail to fill random bytes")?;
    Ok(hex::encode(buf))
}
