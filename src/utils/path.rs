use std::{fs, path::PathBuf, sync::OnceLock};

use directories::ProjectDirs;

pub fn get_project_dir() -> &'static ProjectDirs {
    static I: OnceLock<ProjectDirs> = OnceLock::new();
    I.get_or_init(|| {
        ProjectDirs::from("io.github", "SNRainiar", "mihomosh")
            .expect("fail to get project directories")
    })
}

pub fn get_data_dir() -> &'static PathBuf {
    static I: OnceLock<PathBuf> = OnceLock::new();
    I.get_or_init(|| {
        let dir = get_project_dir().data_local_dir();
        if !dir.is_dir() {
            fs::create_dir_all(&dir).expect("fail to create local data directory")
        }

        dir.into()
    })
}

pub fn get_profile_conf_dir() -> &'static PathBuf {
    static I: OnceLock<PathBuf> = OnceLock::new();
    I.get_or_init(|| {
        let dir = get_data_dir().join("profile_conf");
        if !dir.is_dir() {
            fs::create_dir_all(&dir).expect("fail to create profile config directory")
        }

        dir
    })
}

pub fn get_profile_data_dir() -> &'static PathBuf {
    static I: OnceLock<PathBuf> = OnceLock::new();
    I.get_or_init(|| {
        let dir = get_data_dir().join("profile_data");
        if !dir.is_dir() {
            fs::create_dir_all(&dir).expect("fail to create profile data directory")
        }

        dir
    })
}
