use std::fs;

use anyhow::{Context, Result};

use crate::{
    models::config::Config,
    println_secondary, println_success,
    utils::{file, prompt},
};

pub fn view(viewer: String) -> Result<()> {
    let path = Config::get_path();
    file::view_file(&viewer, path).with_context(|| {
        format!(
            "Fail to view file `{}` with viewer `{}`",
            path.display(),
            viewer
        )
    })?;
    Ok(())
}

pub fn edit(editor: String) -> Result<()> {
    // Edit & verify
    let path = Config::get_path();
    let contents = fs::read_to_string(path)
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

    // Update conigs
    Config::update(&contents).context("Fail to update config file")?;
    println_success!("Mihomo configs saved");
    Ok(())
}

pub fn reset() -> Result<()> {
    // Confirm to reset
    let input = prompt::confirm("Are you sure to reset the Mihomosh configs?")
        .context("Fail to show confirm prompt")?;
    if !input {
        println_secondary!("Skipped");
        return Ok(());
    }

    // Reset configs
    Config::reset().context("Fail to reset config file")?;
    println_success!("Mihomo configs reset");
    Ok(())
}
