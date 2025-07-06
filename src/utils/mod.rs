pub mod api;
pub mod file;
pub mod macros;

use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    sync::OnceLock,
};

use anyhow::{Context, Result};
use directories::ProjectDirs;

use crate::str_primary;

pub fn get_data_dir() -> &'static PathBuf {
    static INSTANCE: OnceLock<PathBuf> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let dir = ProjectDirs::from("io.github", "SNRainiar", "mihomosh")
            .expect("fail to get data directory");
        let dir = dir.data_local_dir();
        if !dir.is_dir() {
            fs::create_dir_all(dir)
                .with_context(|| format!("path: {}", dir.display()))
                .expect("fail to create data directory");
        }

        dir.to_owned()
    })
}

pub fn prompt<S: AsRef<str>>(prompt: S) -> Result<String> {
    print!("{}", str_primary!("{}", prompt.as_ref()));
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_owned())
}
