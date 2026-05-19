use std::{fs, path::PathBuf, sync::OnceLock};

use directories::ProjectDirs;
use eyre::Context;

pub fn get_data_dir() -> &'static PathBuf {
    static INSTANCE: OnceLock<PathBuf> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let dir = ProjectDirs::from("io.github", "SamuNatsu", "mihomosh")
            .expect("the home directory path should be retrievable")
            .data_local_dir()
            .to_owned();

        if !dir.is_dir() {
            fs::create_dir_all(&dir)
                .wrap_err_with(|| format!("fail to create directory `{}`", dir.display()))
                .expect("the project data directory should be creatable");
        }

        dir
    })
}

pub fn get_profile_dir() -> &'static PathBuf {
    static INSTANCE: OnceLock<PathBuf> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let dir = get_data_dir().join("profiles");
        if !dir.is_dir() {
            fs::create_dir_all(&dir)
                .wrap_err_with(|| format!("fail to create directory `{}`", dir.display()))
                .expect("the profile directory should be creatable");
        }

        dir
    })
}
