use std::fs;

use anyhow::{Context, Result};

use crate::{
    println_secondary, println_success,
    utils::{dir, file, prompt},
};

const DEFAULT_EXTEND_CONFIG_TEMPLATE: &'static str =
    include_str!("../includes/default_extend_config.yaml");
const DEFAULT_EXTEND_SCRIPT_TEMPLATE: &'static str =
    include_str!("../includes/default_extend_script.js");

pub fn view_ext_conf(viewer: String) -> Result<()> {
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

pub fn view_ext_script(viewer: String) -> Result<()> {
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

pub fn edit_ext_conf(editor: String) -> Result<()> {
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

pub fn edit_ext_script(editor: String) -> Result<()> {
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
