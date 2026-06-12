use std::{
    ffi::OsStr,
    ops::{Deref, DerefMut},
    path::Path,
};

use eyre::{Context, Result};
use tempfile::{NamedTempFile, TempPath};
use tokio::{fs::File, task};

pub struct TempFile {
    file: File,
    path: TempPath,
}

impl Deref for TempFile {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl DerefMut for TempFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}

impl TempFile {
    pub async fn create<S: AsRef<OsStr>>(suffix: S) -> Result<Self> {
        // Create named temporary file
        let suf = format!(".{}", suffix.as_ref().to_string_lossy());
        let temp_file = task::spawn_blocking(move || NamedTempFile::with_suffix(suf))
            .await
            .wrap_err("failed to spawn blocking task for creating named temporary file")?
            .wrap_err_with(|| {
                format!(
                    "failed to create named temporary file with suffix `{}`",
                    suffix.as_ref().to_string_lossy()
                )
            })?;

        // Extract members
        let (file, path) = temp_file.into_parts();
        let file = File::from_std(file);

        // Done
        Ok(Self { path, file })
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
}
