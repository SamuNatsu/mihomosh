use anyhow::{Context, Result};

use crate::{
    models::{meta::Meta, profile::Profile},
    println_secondary,
    utils::file,
};

pub fn view_info(uuid_or_name: String, viewer: String) -> Result<()> {
    // Get path
    let uuid = Meta::find_uuid_or_name(&uuid_or_name)
        .with_context(|| format!("Fail to find UUID or name `{}`", uuid_or_name))?;
    let path = Profile::get_path(&uuid);

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

pub fn view_file(uuid_or_name: String, viewer: String) -> Result<()> {
    // Get path
    let uuid = Meta::find_uuid_or_name(&uuid_or_name)
        .with_context(|| format!("Fail to find UUID or name `{}`", uuid_or_name))?;
    let path = Profile::get_data_path(&uuid);

    // Check file
    if !path.is_file() {
        println_secondary!("No file data, please edit or update the profile");
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

pub fn view_ext_conf(uuid_or_name: String, viewer: String) -> Result<()> {
    // Get path
    let uuid = Meta::find_uuid_or_name(&uuid_or_name)
        .with_context(|| format!("Fail to find UUID or name `{}`", uuid_or_name))?;
    let path = Profile::get_ext_conf_path(&uuid);

    // Check file
    if !path.is_file() {
        println_secondary!("No extend configs");
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

pub fn view_ext_script(uuid_or_name: String, viewer: String) -> Result<()> {
    // Get path
    let uuid = Meta::find_uuid_or_name(&uuid_or_name)
        .with_context(|| format!("Fail to find UUID or name `{}`", uuid_or_name))?;
    let path = Profile::get_ext_script_path(&uuid);

    // Check file
    if !path.is_file() {
        println_secondary!("No extend script");
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
