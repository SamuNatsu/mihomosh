use std::{fs, path::PathBuf, sync::OnceLock};

use directories::ProjectDirs;

pub fn get_data_dir() -> &'static PathBuf {
    static INSTANCE: OnceLock<PathBuf> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let dir = ProjectDirs::from("io.github", "SNRainiar", "mihomosh")
            .expect("fail to get data directory");
        let dir = dir.data_local_dir();
        if !dir.is_dir() {
            fs::create_dir_all(dir).expect("fail to create data directory");
        }

        dir.to_owned()
    })
}

pub fn get_profile_dir() -> &'static PathBuf {
    static INSTANCE: OnceLock<PathBuf> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let dir = get_data_dir().join("profiles");
        if !dir.is_dir() {
            fs::create_dir_all(&dir).expect("fail to create profile directory");
        }

        dir
    })
}
