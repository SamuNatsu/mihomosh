use anyhow::{Context, Result};

use crate::{
    includes::{DEFAULT_EXTEND_CONFIG_TEMPLATE, DEFAULT_EXTEND_SCRIPT_TEMPLATE},
    println_danger, println_primary, println_secondary, println_success,
    utils::{dir, file},
};

pub fn view_ext_conf(viewer: String) -> Result<()> {
    let path = dir::get_data_dir().join("extend.yaml");
    let exists = file::view_file(&path, &viewer).with_context(|| {
        format!(
            "Fail to view file `{}` with viewer `{viewer}`",
            path.display()
        )
    })?;
    if !exists {
        println_secondary!("No global extend configs");
    }

    Ok(())
}

pub fn view_ext_script(viewer: String) -> Result<()> {
    let path = dir::get_data_dir().join("extend.js");
    let exists = file::view_file(&path, &viewer).with_context(|| {
        format!(
            "Fail to view file `{}` with viewer `{viewer}`",
            path.display()
        )
    })?;
    if !exists {
        println_secondary!("No global extend script");
    }

    Ok(())
}

pub async fn edit_ext_conf(editor: Option<String>) -> Result<()> {
    let path = dir::get_data_dir().join("extend.yaml");
    let saved = file::edit_file(&path, editor, Some(DEFAULT_EXTEND_CONFIG_TEMPLATE))
        .with_context(|| format!("Fail to edit file `{}`", path.display()))?;
    if saved {
        println_success!("Global extend config saved");
    }

    // Try reactivate
    println_primary!("Reactivating last activated profile...");
    if let Err(err) = super::activate(None).await {
        println_danger!("{err:?}");
    }

    // Success
    Ok(())
}

pub async fn edit_ext_script(editor: Option<String>) -> Result<()> {
    let path = dir::get_data_dir().join("extend.js");
    let saved = file::edit_file(&path, editor, Some(DEFAULT_EXTEND_SCRIPT_TEMPLATE))
        .with_context(|| format!("Fail to edit file `{}`", path.display()))?;
    if saved {
        println_success!("Global extend scripts saved");
    }

    // Try reactivate
    println_primary!("Reactivating last activated profile...");
    if let Err(err) = super::activate(None).await {
        println_danger!("{err:?}");
    }

    // Success
    Ok(())
}
