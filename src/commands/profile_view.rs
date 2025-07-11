use anyhow::{Context, Result, bail};

use crate::{
    models::{meta::Meta, profile::Profile},
    println_secondary,
    utils::file,
};

pub fn view_info(uuid_or_name: String, viewer: String) -> Result<()> {
    let uuid = Meta::find_uuid_or_name(&uuid_or_name)
        .with_context(|| format!("Fail to find UUID or name `{uuid_or_name}`"))?;
    let path = Profile::get_path(&uuid);
    let exists = file::view_file(&path, &viewer).with_context(|| {
        format!(
            "Fail to view file `{}` with viewer `{viewer}`",
            path.display()
        )
    })?;
    if !exists {
        bail!("File `{}` not exists", path.display());
    }

    Ok(())
}

pub fn view_file(uuid_or_name: String, viewer: String) -> Result<()> {
    // Get path
    let uuid = Meta::find_uuid_or_name(&uuid_or_name)
        .with_context(|| format!("Fail to find UUID or name `{uuid_or_name}`"))?;
    let path = Profile::get_data_path(&uuid);
    let exists = file::view_file(&path, &viewer).with_context(|| {
        format!(
            "Fail to view file `{}` with viewer `{viewer}`",
            path.display()
        )
    })?;
    if !exists {
        println_secondary!("No file data, please edit or update the profile");
    }

    Ok(())
}

pub fn view_ext_conf(uuid_or_name: String, viewer: String) -> Result<()> {
    let uuid = Meta::find_uuid_or_name(&uuid_or_name)
        .with_context(|| format!("Fail to find UUID or name `{uuid_or_name}`"))?;
    let path = Profile::get_ext_conf_path(&uuid);
    let exists = file::view_file(&path, &viewer).with_context(|| {
        format!(
            "Fail to view file `{}` with viewer `{viewer}`",
            path.display()
        )
    })?;
    if !exists {
        println_secondary!("No extend configs");
    }

    Ok(())
}

pub fn view_ext_script(uuid_or_name: String, viewer: String) -> Result<()> {
    let uuid = Meta::find_uuid_or_name(&uuid_or_name)
        .with_context(|| format!("Fail to find UUID or name `{uuid_or_name}`"))?;
    let path = Profile::get_ext_script_path(&uuid);
    let exists = file::view_file(&path, &viewer).with_context(|| {
        format!(
            "Fail to view file `{}` with viewer `{viewer}`",
            path.display()
        )
    })?;
    if !exists {
        println_secondary!("No extend script");
    }

    Ok(())
}
