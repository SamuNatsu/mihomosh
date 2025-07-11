use std::fs;

use anyhow::{Context, Result};

use crate::{
    includes::{DEFAULT_EXTEND_CONFIG_TEMPLATE, DEFAULT_EXTEND_SCRIPT_TEMPLATE},
    models::{
        meta::Meta,
        profile::{Profile, ProfileType},
    },
    println_secondary, println_success,
    utils::{file, prompt},
};

pub fn edit_info(uuid_or_name: String, editor: String) -> Result<()> {
    // Edit profile
    let uuid = Meta::find_uuid_or_name(&uuid_or_name)
        .with_context(|| format!("Fail to find UUID or name `{uuid_or_name}`"))?;
    let path = Profile::get_path(&uuid);
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Fail to read file `{}`", path.display()))?;
    let contents = file::edit_temp_file(".yaml", &editor, &contents)
        .with_context(|| format!("Fail to edit temporary file with editor `{editor}`"))?;

    // Verify profile
    let profile = serde_yml::from_str::<Profile>(&contents).context("Fail to parse profile")?;
    profile.verify().context("Fail to verify profile")?;

    // Confirm to save
    let input = prompt::confirm("Are you sure to save the changes?")
        .context("Fail to show confirm prompt")?;
    if !input {
        println_secondary!("Changes discarded");
        return Ok(());
    }

    // Update metadata
    let mut meta_map = Meta::get_instance().lock().unwrap();
    let old_meta = meta_map.get(&uuid).unwrap().clone();
    meta_map.insert(
        uuid.clone(),
        Meta {
            name: profile.name.clone().trim().to_string(),
            is_remote: if let ProfileType::Local = profile.r#type {
                false
            } else {
                true
            },
            ..old_meta
        },
    );

    drop(meta_map);
    Meta::flush().context("Fail to flush metadata")?;

    // Update profile
    let path = Profile::get_path(&uuid);
    fs::write(&path, contents)
        .with_context(|| format!("Fail to write file `{}`", path.display()))?;

    // Success
    println_success!("Profile `{}` with UUID `{uuid}` saved", profile.name);
    Ok(())
}

pub fn edit_file(uuid_or_name: String, editor: String) -> Result<()> {
    let uuid = Meta::find_uuid_or_name(&uuid_or_name)
        .with_context(|| format!("Fail to find UUID or name `{uuid_or_name}`"))?;
    let name = Meta::get_instance()
        .lock()
        .unwrap()
        .get(&uuid)
        .unwrap()
        .name
        .clone();
    let path = Profile::get_data_path(&uuid);
    let saved = file::edit_file(&path, &editor, Some(""))
        .with_context(|| format!("Fail to edit file `{}`", path.display()))?;
    if saved {
        println_success!("Profile `{name}` with UUID `{uuid}` file data saved");
    }

    Ok(())
}

pub fn edit_ext_conf(uuid_or_name: String, editor: String) -> Result<()> {
    let uuid = Meta::find_uuid_or_name(&uuid_or_name)
        .with_context(|| format!("Fail to find UUID or name `{uuid_or_name}`"))?;
    let name = Meta::get_instance()
        .lock()
        .unwrap()
        .get(&uuid)
        .unwrap()
        .name
        .clone();
    let path = Profile::get_ext_conf_path(&uuid);
    let saved = file::edit_file(&path, &editor, Some(DEFAULT_EXTEND_CONFIG_TEMPLATE))
        .with_context(|| format!("Fail to edit file `{}`", path.display()))?;
    if saved {
        println_success!("Profile `{name}` with UUID `{uuid}` extend config saved");
    }

    Ok(())
}

pub fn edit_ext_script(uuid_or_name: String, editor: String) -> Result<()> {
    let uuid = Meta::find_uuid_or_name(&uuid_or_name)
        .with_context(|| format!("Fail to find UUID or name `{uuid_or_name}`"))?;
    let name = Meta::get_instance()
        .lock()
        .unwrap()
        .get(&uuid)
        .unwrap()
        .name
        .clone();
    let path = Profile::get_ext_script_path(&uuid);
    let saved = file::edit_file(&path, &editor, Some(DEFAULT_EXTEND_SCRIPT_TEMPLATE))
        .with_context(|| format!("Fail to edit file `{}`", path.display()))?;
    if saved {
        println_success!("Profile `{name}` with UUID `{uuid}` extend script saved");
    }

    Ok(())
}
