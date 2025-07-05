use std::fs;

use anyhow::Result;

use crate::{
    models::config::Config,
    normal, success,
    utils::{self, file},
};

pub fn view(viewer: String) -> Result<()> {
    let path = Config::get_path();
    file::view_file(&viewer, path)?;
    Ok(())
}

pub fn edit(editor: String) -> Result<()> {
    // Edit & verify
    let path = Config::get_path();
    let contents = fs::read_to_string(path)?;
    let contents = file::edit_temp_file(".yaml", &editor, &contents)?;
    Config::verify(&contents)?;

    // Confirm to save
    let input = utils::prompt("Are you sure to save the changes? (y/N) ")?;
    if input.to_lowercase() != "y" {
        return normal!("No changes");
    }

    // Write configs
    fs::write(path, &contents)?;
    success!("Mihomo configs saved")
}

pub fn reset() -> Result<()> {
    // Confirm to reset
    let input = utils::prompt("Are you sure to reset the Mihomosh configs? (y/N) ")?;
    if input.to_lowercase() != "y" {
        return normal!("No changes");
    }

    // Reset configs
    Config::reset()?;
    success!("Mihomo configs reset")
}
