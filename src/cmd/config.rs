use std::fs;

use anyhow::{Context, Result};

use crate::{
    data::config::Config,
    utils::{
        file, prompt,
        result::{normal, success},
    },
};

pub fn edit(editor: Option<String>) -> Result<()> {
    let cfg = Config::get_instance();

    // Edit config file contents
    let path = cfg.get_path();
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("try to read file `{}`", path.display()))?;
    let contents =
        file::edit_temp_file(".yaml", editor.or(Some(cfg.editor.clone())), Some(contents))
            .with_context(|| "try to edit temporary contents")?;
    serde_yaml::from_str::<Config>(&contents).with_context(|| "try to parse temporary contents")?;

    // Confirm to save
    if !prompt::confirm("Are you sure to save the new configuration?")
        .with_context(|| "try to show confirm prompt")?
    {
        return normal!("Nothing changed");
    }

    // Save configs
    fs::write(&path, contents)
        .with_context(|| format!("try to write file `{}`", path.display()))?;

    // Success
    success!("Configuration edited")
}

pub fn reset() -> Result<()> {
    let cfg = Config::get_instance();

    // Confirm to reset
    if !prompt::confirm("Are you sure to reset the configurations?")
        .with_context(|| "try to show confirm prompt")?
    {
        return normal!("Nothing changed");
    }

    // Reset configs
    let path = cfg.get_path();
    fs::write(&path, Config::DEFAULT_CONFIG)
        .with_context(|| format!("try to write file `{}`", path.display()))?;

    // Success
    success!("Configurations reset")
}

pub fn view() -> Result<()> {
    let cfg = Config::get_instance();

    // Show configs
    let path = cfg.get_path();
    file::show_file(&path).with_context(|| format!("try to show file `{}`", path.display()))?;

    // Success
    Ok(())
}
