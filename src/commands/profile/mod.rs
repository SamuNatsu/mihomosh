mod edit;
mod global;
mod view;

use std::fs;

use anyhow::{Context, Result, bail};
use chrono::Utc;
use rand::{TryRngCore, rngs::OsRng};
use tokio::task::JoinSet;

use crate::{
    arguments::profile::ProfileArgs,
    includes::DEFAULT_PROFILE_TEMPLATE,
    models::{
        meta::Meta,
        profile::{Profile, ProfileType, SubUserInfo},
    },
    println_danger, println_primary, println_secondary, println_success,
    utils::{dir, file, prompt},
};

pub async fn handle_profile(args: ProfileArgs) -> Result<()> {
    match args {
        ProfileArgs::Update { uuid_or_name } => update(uuid_or_name).await?,
        ProfileArgs::Activate { uuid_or_name } => activate(uuid_or_name).await?,
        ProfileArgs::Create { editor } => create(editor)?,
        ProfileArgs::Delete { uuid_or_name } => delete(uuid_or_name)?,
        ProfileArgs::List => list()?,
        ProfileArgs::View(args) => view::handle_view(args)?,
        ProfileArgs::Edit(args) => edit::handle_edit(args)?,
        ProfileArgs::ViewGlobalExtendConfig { viewer } => global::view_ext_conf(viewer)?,
        ProfileArgs::ViewGlobalExtendScript { viewer } => global::view_ext_script(viewer)?,
        ProfileArgs::EditGlobalExtendConfig { editor } => global::edit_ext_conf(editor).await?,
        ProfileArgs::EditGlobalExtendScript { editor } => global::edit_ext_script(editor).await?,
    }
    Ok(())
}

async fn update(uuid_or_name: Option<String>) -> Result<()> {
    let uuid = match uuid_or_name {
        Some(uuid_or_name) => vec![
            Meta::find_uuid_or_name(&uuid_or_name)
                .with_context(|| format!("Fail to find UUID or name `{uuid_or_name}`"))?,
        ],
        None => Meta::get_instance()
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, v)| v.is_remote)
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>(),
    };
    if uuid.is_empty() {
        println_secondary!("No profile to be updated");
        return Ok(());
    }
    println_secondary!("{} profile(s) to be updated", uuid.len());

    // Create tasks
    let mut set = JoinSet::new();
    for uuid in uuid {
        set.spawn(async {
            let uuid = uuid;
            let res: Result<SubUserInfo> = async {
                let profile = Profile::load(&uuid)
                    .with_context(|| format!("Fail to load profile with UUID `{uuid}`"))?;
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

    // Resolve tasks
    let res = set.join_all().await;
    {
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
    }

    Meta::flush().context("Fail to flush metadata")?;

    // Try reactivate
    println_primary!("Reactivating last activated profile...");
    if let Err(err) = activate(None).await {
        println_danger!("{err:?}");
    }

    // Success
    Ok(())
}

pub async fn activate(uuid_or_name: Option<String>) -> Result<()> {
    // Get profile
    let uuid = match uuid_or_name {
        Some(uuid_or_name) => Meta::find_uuid_or_name(&uuid_or_name)
            .with_context(|| format!("Fail to find UUID or name `{uuid_or_name}`"))?,
        None => {
            let path = dir::get_data_dir().join("last-profile");
            if !path.is_file() {
                bail!("No last activated profile, please activate some profile first");
            }
            let uuid = fs::read_to_string(&path)
                .with_context(|| format!("Fail to read file `{}`", path.display()))?;
            Meta::find_uuid_or_name(&uuid).with_context(|| format!("Fail to find UUID `{uuid}`"))?
        }
    };
    let profile =
        Profile::load(&uuid).with_context(|| format!("Fail to load profile with UUID `{uuid}`"))?;

    // Activate profile
    println_primary!(
        "Activating profile `{}` with UUID `{uuid}`...",
        profile.name
    );
    profile
        .activate(&uuid)
        .await
        .with_context(|| format!("Fail to activate profile with UUID `{uuid}`"))?;
    println_success!("Profile activated");

    // Save last activated
    let path = dir::get_data_dir().join("last-profile");
    fs::write(&path, &uuid).with_context(|| format!("Fail to write file `{}`", path.display()))?;

    // Success
    Ok(())
}

fn create(editor: Option<String>) -> Result<()> {
    let mut meta_map = Meta::get_instance().lock().unwrap();

    // Edit temporary file
    let contents = file::edit_temp_file(
        ".yaml",
        editor,
        DEFAULT_PROFILE_TEMPLATE.replace("<CARGO_PKG_VERSION>", env!("CARGO_PKG_VERSION")),
    )
    .context("Fail to edit temporary file")?;

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

fn delete(uuid_or_name: String) -> Result<()> {
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
    let path = vec![
        Profile::get_path(&uuid),
        Profile::get_data_path(&uuid),
        Profile::get_ext_conf_path(&uuid),
        Profile::get_ext_script_path(&uuid),
    ];
    for path in path {
        if path.is_file() {
            fs::remove_file(&path)
                .with_context(|| format!("Fail to remove file `{}`", path.display()))?;
        }
    }

    // Update metadata
    meta_map.remove(&uuid);

    drop(meta_map);
    Meta::flush().context("Fail to flush metadata")?;

    // Success
    println_success!("Profile `{name}` with UUID `{uuid}` deleted");
    Ok(())
}

fn list() -> Result<()> {
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
