pub mod config;
pub mod profile;

use std::{fs, path::PathBuf, sync::LazyLock};

use directories::ProjectDirs;
use eyre::Context;

pub static DATA_LOCAL_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    // Get data local directory path
    let path = ProjectDirs::from("io.github", "SamuNatsu", "mihomosh")
        .expect("the home directory path should be retrievable")
        .data_local_dir()
        .to_owned();

    // Create directories if not exists
    if !path.is_dir() {
        fs::create_dir_all(&path)
            .wrap_err_with(|| format!("fail to create directory `{}`", path.display()))
            .expect("the project data directory should be creatable");
    }

    // Return path
    path
});
