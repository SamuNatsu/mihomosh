use std::fs;

use anyhow::{Context, Result, bail};

use crate::{
    models::config::Config,
    println_secondary, println_success,
    utils::{file, prompt},
};

pub fn view(viewer: String) -> Result<()> {
    let path = Config::get_path();
    let exists = file::view_file(path, &viewer).with_context(|| {
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

pub fn edit(editor: String) -> Result<()> {
    // Edit configs
    let path = Config::get_path();
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Fail to read file `{}`", path.display()))?;
    let contents = file::edit_temp_file(".yaml", &editor, &contents)
        .with_context(|| format!("Fail to edit temporary file with editor `{editor}`"))?;

    // Confirm to save
    let input = prompt::confirm("Are you sure to save the changes?")
        .context("Fail to show confirm prompt")?;
    if !input {
        println_secondary!("Changes discarded");
        return Ok(());
    }

    // Update configs
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
