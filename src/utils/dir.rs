use std::{fs, path::PathBuf, sync::OnceLock};

use anyhow::{Context, anyhow};
use directories::ProjectDirs;

pub fn get_data_dir() -> &'static PathBuf {
    static INSTANCE: OnceLock<PathBuf> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let dir = ProjectDirs::from("io.github", "SamuNatsu", "mihomosh")
            .ok_or(anyhow!("Fail to get project directory"))
            .unwrap()
            .data_local_dir()
            .to_owned();
        if !dir.is_dir() {
            fs::create_dir_all(&dir)
                .with_context(|| format!("Fail to create directory `{}`", dir.display()))
                .unwrap();
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
                .with_context(|| format!("Fail to create directory `{}`", dir.display()))
                .unwrap();
        }

        dir
    })
}
