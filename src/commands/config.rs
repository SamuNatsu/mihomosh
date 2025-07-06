use std::fs;

use anyhow::Result;

use crate::{
    models::config::Config,
    println_secondary, println_success,
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
        println_secondary!("Changes discarded");
        return Ok(());
    }

    // Write configs
    fs::write(path, &contents)?;
    println_success!("Mihomo configs saved");
    Ok(())
}

pub fn reset() -> Result<()> {
    // Confirm to reset
    let input = utils::prompt("Are you sure to reset the Mihomosh configs? (y/N) ")?;
    if input.to_lowercase() != "y" {
        println_secondary!("Skipped");
        return Ok(());
    }

    // Reset configs
    Config::reset()?;
    println_success!("Mihomo configs reset");
    Ok(())
}
