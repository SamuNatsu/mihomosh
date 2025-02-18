use std::fs;

use anyhow::{bail, Context, Result};
use chrono::Utc;
use tokio::task::JoinSet;

use crate::{
    data::{
        config::Config,
        profile::{Config as ProfileConfig, ConfigType as ProfileConfigType, Meta, Metas},
    },
    utils::{
        self, file, path, prompt,
        result::{normal, success},
    },
};

const DEFAULT_CONFIG_TEMPLATE: &'static str = include_str!("../includes/default_profile.yaml");

pub async fn activate(uuid_or_name: String) -> Result<()> {
    // Get profile metadatas
    let metas = Metas::get_instance().lock().unwrap();

    // Get profile metadata
    let meta = metas
        .try_get_meta(&uuid_or_name)
        .with_context(|| format!("try to get profile metadata by `{}`", uuid_or_name))?;

    // Get profile config
    let conf = meta
        .try_get_conf()
        .with_context(|| format!("try to get profile config by UUID `{}`", meta.uuid))?;

    // Activate profile
    conf.activate()
        .await
        .with_context(|| format!("try to activate profile by UUID `{}`", meta.uuid))?;

    // Success
    success!(
        "Profile `{}` with UUID `{}` activated",
        meta.name,
        meta.uuid
    )
}

pub fn delete(uuid_or_name: String) -> Result<()> {
    // Get profile metadata map
    let mut metas = Metas::get_instance().lock().unwrap();

    // Get profile metadata
    let meta = metas
        .try_get_meta(&uuid_or_name)
        .with_context(|| format!("try to get profile metadata by `{}`", uuid_or_name))?
        .clone();

    // Confirm to delete
    let prompt = format!(
        "Are you sure to delete the profile `{}` with UUID `{}`?",
        meta.name, meta.uuid
    );
    if !prompt::confirm(&prompt).with_context(|| "try to show confirm prompt")? {
        return normal!("Nothing changed");
    }

    // Update metadata
    metas.remove(&meta.uuid);
    metas
        .flush()
        .with_context(|| "try to flush profile metadatas")?;

    // Delete config file
    let path = path::get_profile_conf_dir().join(format!("{}.yaml", meta.uuid));
    fs::remove_file(&path).with_context(|| format!("try to delete file `{}`", path.display()))?;

    // Delete data file
    let path = path::get_profile_data_dir().join(format!("{}.yaml", meta.uuid));
    if path.is_file() {
        fs::remove_file(&path)
            .with_context(|| format!("try to delete file `{}`", path.display()))?;
    }

    // Success
    success!("Profile `{}` with UUID `{}` deleted", meta.name, meta.uuid)
}

pub fn edit_conf(uuid_or_name: String) -> Result<()> {
    let cfg = Config::get_instance();

    // Get profile metadata map
    let mut metas = Metas::get_instance().lock().unwrap();

    // Get profile metadata
    let meta = metas
        .try_get_meta(&uuid_or_name)
        .with_context(|| format!("try to get profile metadata by `{}`", uuid_or_name))?
        .clone();

    // Edit profile configs
    let path = path::get_profile_conf_dir().join(format!("{}.yaml", meta.uuid));
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("try to read file `{}`", path.display()))?;
    let contents = file::edit_temp_file(".yaml", Some(&cfg.editor), Some(&contents))
        .with_context(|| "try to edit temporary contents")?;
    let value = serde_yaml::from_str::<ProfileConfig>(&contents)
        .with_context(|| "try to parse temporary contents")?;
    value
        .verify()
        .with_context(|| "try to verify temporary contents")?;

    // Confirm to save
    let prompt = format!(
        "Are you sure to save the profile configurations `{}` with UUID `{}`?",
        value.name, meta.uuid
    );
    if !prompt::confirm(&prompt).with_context(|| "try to show confirm prompt")? {
        return normal!("Nothing changed");
    }

    // Update metadata
    let old = metas.get(&meta.uuid).unwrap().clone();
    metas.insert(
        meta.uuid.clone(),
        Meta {
            name: value.name.clone(),
            remote: if let ProfileConfigType::Local = value.r#type {
                false
            } else {
                true
            },
            expired_at: None,
            total_bytes: None,
            ..old
        },
    );
    metas.flush().with_context(|| "try to flush MetadataMap")?;

    // Update config file
    fs::write(&path, contents)
        .with_context(|| format!("try to write file `{}`", path.display()))?;

    // Success
    success!(
        "Profile configurations `{}` with UUID `{}` edited",
        value.name,
        meta.uuid
    )
}

pub fn edit_data(uuid_or_name: String) -> Result<()> {
    let cfg = Config::get_instance();

    // Get profile metadata map
    let metas = Metas::get_instance().lock().unwrap();

    // Get profile metadata
    let meta = metas
        .try_get_meta(&uuid_or_name)
        .with_context(|| format!("try to get profile metadata by `{}`", uuid_or_name))?;

    // Edit profile data
    let path = path::get_profile_data_dir().join(format!("{}.yaml", meta.uuid));
    let contents = if path.is_file() {
        fs::read_to_string(&path)
            .with_context(|| format!("try to read file `{}`", path.display()))?
    } else {
        String::new()
    };
    let contents = file::edit_temp_file(".yaml", Some(&cfg.editor), Some(&contents))
        .with_context(|| "try to edit temporary contents")?;

    // Confirm to save
    let prompt = format!(
        "Are you sure to save the profile data `{}` with UUID `{}`?",
        meta.name, meta.uuid
    );
    if !prompt::confirm(&prompt).with_context(|| "try to show confirm prompt")? {
        return normal!("Nothing changed");
    }

    // Update data file
    fs::write(&path, contents)
        .with_context(|| format!("try to write file `{}`", path.display()))?;

    // Success
    success!(
        "Profile data `{}` with UUID `{}` edited",
        meta.name,
        meta.uuid
    )
}

pub fn list() -> Result<()> {
    let metas = Metas::get_instance().lock().unwrap();

    // If no profile
    if metas.len() == 0 {
        return normal!("No profiles found");
    }

    // Get profile list
    let mut kv = metas.iter().collect::<Vec<_>>();
    kv.sort_by(|a, b| {
        if a.1.name == b.1.name {
            a.0.cmp(b.0)
        } else {
            a.1.name.cmp(&b.1.name)
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

pub fn new() -> Result<()> {
    let cfg = Config::get_instance();
    let mut metas = Metas::get_instance().lock().unwrap();

    // Edit temporary file
    let contents = file::edit_temp_file(
        ".yaml",
        Some(&cfg.editor),
        Some(DEFAULT_CONFIG_TEMPLATE.replace("<CARGO_PKG_VERSION>", env!("CARGO_PKG_VERSION"))),
    )
    .with_context(|| "try to edit temporary contents")?;
    let value = serde_yaml::from_str::<ProfileConfig>(&contents)
        .with_context(|| "try to parse temporary contents")?;
    value
        .verify()
        .with_context(|| "try to verify the temporary contents")?;

    // Confirm to create
    let prompt = format!("Are you sure to create the new profile `{}`?", value.name);
    if !prompt::confirm(&prompt).with_context(|| "try to show confirm prompt")? {
        return normal!("Nothing changed");
    }

    // Update metadata
    let uuid = utils::gen_uuid();
    metas.insert(
        uuid.clone(),
        Meta {
            uuid: uuid.clone(),
            name: value.name.clone(),
            remote: if let ProfileConfigType::Local = value.r#type {
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
    metas.flush().with_context(|| "try to flush MetadataMap")?;

    // Update config file
    let path = path::get_profile_conf_dir().join(format!("{}.yaml", uuid));
    fs::write(&path, contents)
        .with_context(|| format!("try to write file `{}`", path.display()))?;

    // Success
    success!("New profile `{}` with UUID `{}` added", value.name, uuid)
}

pub async fn update(uuid_or_name: Option<String>) -> Result<()> {
    let mut metas = Metas::get_instance().lock().unwrap();

    // If update specific profile
    if let Some(uuid_or_name) = uuid_or_name {
        // Get profile metadata
        let mut meta = metas
            .try_get_meta(&uuid_or_name)
            .with_context(|| format!("try to get profile metadata by `{}`", uuid_or_name))?
            .clone();

        // Load config
        let path = path::get_profile_conf_dir().join(format!("{}.yaml", meta.uuid));
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("try to read file `{}`", path.display()))?;
        let profile = serde_yaml::from_str::<ProfileConfig>(&contents)
            .with_context(|| format!("try to parse profile config `{}`", path.display()))?;

        // Fetch
        let (used, total, expired_at) = profile
            .fetch()
            .await
            .with_context(|| format!("try to fetch profile data `{}`", path.display()))?;

        // Update metadata
        let name = meta.name.clone();
        meta.used_bytes = used;
        meta.total_bytes = total;
        meta.updated_at = Some(Utc::now().timestamp());
        meta.expired_at = expired_at;
        metas.flush().with_context(|| "try to flush MetadataMap")?;

        // Success
        success!("Profile `{}` with UUID `{}` updated", name, meta.uuid)
    } else {
        // Create tasks
        let mut set = JoinSet::new();
        for uuid in metas.iter().filter(|(_, v)| v.remote).map(|(k, _)| k) {
            let uuid = uuid.clone();
            let path = path::get_profile_conf_dir().join(format!("{}.yaml", uuid));
            let contents = fs::read_to_string(&path)
                .with_context(|| format!("try to read file `{}`", path.display()))?;
            let profile = serde_yaml::from_str::<ProfileConfig>(&contents)
                .with_context(|| format!("try to load profile config `{}`", path.display()))?;
            set.spawn(async move {
                let r = profile.fetch().await;
                match &r {
                    Ok(_) => println!(
                        "{}",
                        console::style(format!(
                            "Profile `{}` with UUID `{}` updated",
                            profile.name, uuid
                        ))
                        .green()
                    ),
                    Err(err) => println!(
                        "{}",
                        console::style(format!(
                            "Profile `{}` with UUID `{}` fail to update: {}",
                            profile.name, uuid, err
                        ))
                        .red()
                    ),
                }

                (uuid, r)
            });
        }

        // Execute tasks
        let r = set.join_all().await;
        let rx = r
            .into_iter()
            .filter(|(_, r)| r.is_ok())
            .map(|(k, v)| (k, v.unwrap()));

        // Update metadata
        for (uuid, (used, total, expired_at)) in rx {
            let meta = metas.get_mut(&uuid).unwrap();
            meta.used_bytes = used;
            meta.total_bytes = total;
            meta.updated_at = Some(Utc::now().timestamp());
            meta.expired_at = expired_at;
        }
        metas.flush().with_context(|| "try to flush MetadataMap")?;

        // Success
        success!("All profiles updated")
    }
}

pub fn view_conf(uuid_or_name: String) -> Result<()> {
    let metas = Metas::get_instance().lock().unwrap();

    // Get profile metadata
    let meta = metas
        .try_get_meta(&uuid_or_name)
        .with_context(|| format!("try to get profile metadata by `{}`", uuid_or_name))?
        .clone();

    // Show file
    let path = path::get_profile_conf_dir().join(format!("{}.yaml", meta.uuid));
    file::show_file(&path).with_context(|| format!("try to show file `{}`", path.display()))?;

    // Success
    Ok(())
}

pub fn view_data(uuid_or_name: String) -> Result<()> {
    let metas = Metas::get_instance().lock().unwrap();

    // Get profile metadata
    let meta = metas
        .try_get_meta(&uuid_or_name)
        .with_context(|| format!("try to get profile metadata by `{}`", uuid_or_name))?
        .clone();

    // Show file
    let path = path::get_profile_data_dir().join(format!("{}.yaml", meta.uuid));
    if !path.is_file() {
        bail!("data not found, maybe you have not edited the local profile's data or updated the remote profile yet");
    }
    file::show_file(&path).with_context(|| format!("try to show file `{}`", path.display()))?;

    // Success
    Ok(())
}

pub fn view_rules(uuid_or_name: String) -> Result<()> {
    let metas = Metas::get_instance().lock().unwrap();

    // Get profile metadata
    let meta = metas
        .try_get_meta(&uuid_or_name)
        .with_context(|| format!("try to get profile metadata by `{}`", uuid_or_name))?
        .clone();

    // Extract rules
    let path = path::get_profile_data_dir().join(format!("{}.yaml", meta.uuid));
    if !path.is_file() {
        bail!("data not found, maybe you have not edited the local profile's data or updated the remote profile yet");
    }
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("try to read file `{}`", path.display()))?;
    let contents = utils::extract_rules(&contents)
        .with_context(|| format!("try to extract rules `{}`", path.display()))?;

    // Show contents
    file::show_contents("activated-rules", &contents, false)
        .with_context(|| "fail to show rules")?;

    // Success
    Ok(())
}
