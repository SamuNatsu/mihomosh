pub mod entity;
pub mod path;
pub mod render;
pub mod table;
pub mod update;

use std::{path::PathBuf, sync::LazyLock};

pub use entity as profile;

pub static GLOB_EXT_CONF_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| super::DATA_LOCAL_DIR.join("ext.yaml"));

pub static GLOB_EXT_SCR_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| super::DATA_LOCAL_DIR.join("ext.js"));
