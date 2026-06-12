use std::{fs, path::PathBuf, sync::LazyLock};

use eyre::Context;

use super::entity::*;

static PROFILE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    // Create profile directory path
    let path = super::super::DATA_LOCAL_DIR.join("profiles");

    // Create directories if not exists
    if !path.is_dir() {
        fs::create_dir_all(&path)
            .wrap_err_with(|| format!("failed to create directory `{}`", path.display()))
            .expect("the profile directory should be creatable");
    }

    // Return path
    path
});

impl Model {
    pub fn get_data_path(&self) -> PathBuf {
        PROFILE_DIR.join(format!("{}.yaml", self.uuid))
    }

    pub fn get_ext_conf_path(&self) -> PathBuf {
        PROFILE_DIR.join(format!("{}.ext.yaml", self.uuid))
    }

    pub fn get_ext_scr_path(&self) -> PathBuf {
        PROFILE_DIR.join(format!("{}.ext.js", self.uuid))
    }
}
