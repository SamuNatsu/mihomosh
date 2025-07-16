use std::fs;

use anyhow::{Context, Result, bail};

use crate::{
    arguments::config::ConfigArgs,
    commands::profile,
    models::config::Config,
    println_danger, println_primary, println_secondary, println_success,
    utils::{file, prompt},
};

pub async fn handle_config(args: ConfigArgs) -> Result<()> {
    match args {
        ConfigArgs::View { viewer } => view(viewer)?,
        ConfigArgs::Edit { editor } => edit(editor).await?,
        ConfigArgs::Reset => reset()?,
    }
    Ok(())
}

fn view(viewer: String) -> Result<()> {
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

async fn edit(editor: String) -> Result<()> {
    // Edit configs
    let path = Config::get_path();
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Fail to read file `{}`", path.display()))?;
    let contents = file::edit_temp_file(".yaml", &editor, &contents)
        .with_context(|| format!("Fail to edit temporary file with editor `{editor}`"))?;

    // Verify configs
    let value = serde_yml::from_str::<Config>(&contents).context("Fail to parse configs")?;
    value.verify().context("Fail to verify configs")?;

    // Confirm to save
    let input = prompt::confirm("Are you sure to save the changes?")
        .context("Fail to show confirm prompt")?;
    if !input {
        println_secondary!("Changes discarded");
        return Ok(());
    }

    // Update configs
    let path = Config::get_path();
    fs::write(path, &contents)
        .with_context(|| format!("Fail to write file `{}`", path.display()))?;
    println_success!("Mihomo configs saved");

    // Try reactivate
    println_primary!("Reactivating last activated profile...");
    if let Err(err) = profile::activate(None).await {
        println_danger!("{err:?}");
    }

    // Success
    Ok(())
}

fn reset() -> Result<()> {
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
