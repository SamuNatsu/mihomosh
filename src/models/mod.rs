pub mod config;
pub mod profile;
pub mod profile_manager;

use std::{fs, path::PathBuf, sync::LazyLock};

#[cfg(not(debug_assertions))]
use directories::ProjectDirs;
use eyre::Context;

pub static DATA_LOCAL_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    // Get data local directory path
    #[cfg(not(debug_assertions))]
    let path = ProjectDirs::from("io.github", "SamuNatsu", "mihomosh")
        .expect("the home directory path should be retrievable")
        .data_local_dir()
        .to_owned();
    #[cfg(debug_assertions)]
    let path = std::env::current_dir()
        .expect("current directory should be valid")
        .join("test_data");

    // Create directories if not exists
    if !path.is_dir() {
        fs::create_dir_all(&path)
            .wrap_err_with(|| format!("failed to create directory `{}`", path.display()))
            .expect("the project data directory should be creatable");
    }

    // Return path
    path
});
