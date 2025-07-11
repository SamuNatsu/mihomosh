use anyhow::{Context, Result};

use crate::{
    includes::{DEFAULT_EXTEND_CONFIG_TEMPLATE, DEFAULT_EXTEND_SCRIPT_TEMPLATE},
    println_secondary, println_success,
    utils::{dir, file},
};

pub fn view_ext_conf(viewer: String) -> Result<()> {
    let path = dir::get_data_dir().join("extend-config.yaml");
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
    let path = dir::get_data_dir().join("extend-script.js");
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

pub fn edit_ext_conf(editor: String) -> Result<()> {
    let path = dir::get_data_dir().join("extend-config.yaml");
    let saved = file::edit_file(&path, &editor, Some(DEFAULT_EXTEND_CONFIG_TEMPLATE))
        .with_context(|| format!("Fail to edit file `{}`", path.display()))?;
    if saved {
        println_success!("Global extend config saved");
    }

    Ok(())
}

pub fn edit_ext_script(editor: String) -> Result<()> {
    let path = dir::get_data_dir().join("extend-script.js");
    let saved = file::edit_file(&path, &editor, Some(DEFAULT_EXTEND_SCRIPT_TEMPLATE))
        .with_context(|| format!("Fail to edit file `{}`", path.display()))?;
    if saved {
        println_success!("Global extend scripts saved");
    }

    Ok(())
}
