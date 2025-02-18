use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::{
    data::config::Config,
    utils::{self, file},
};

pub fn profile() -> Result<()> {
    let cfg = Config::get_instance();

    // Show profile
    let path = Path::new(&cfg.mihomo_path)
        .canonicalize()
        .with_context(|| format!("try to canonicalize path `{}`", cfg.mihomo_path))?;
    file::show_file(&path).with_context(|| format!("try to show file `{}`", path.display()))?;

    // Success
    Ok(())
}

pub fn rules() -> Result<()> {
    let cfg = Config::get_instance();

    // Extract rules
    let path = Path::new(&cfg.mihomo_path)
        .canonicalize()
        .with_context(|| format!("try to canonicalize path `{}`", cfg.mihomo_path))?;
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("try to read file `{}`", path.display()))?;
    let contents = utils::extract_rules(&contents)
        .with_context(|| format!("try to extract rules `{}`", path.display()))?;

    // Show contents
    file::show_contents("activated-rules", &contents, false)
        .with_context(|| "fail to show rules")?;

    // Success
    Ok(())
}
