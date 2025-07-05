use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::Command,
};

use anyhow::{Result, anyhow, bail};
use tempfile::NamedTempFile;

pub fn edit_temp_file<S1, S2, S3>(suffix: S1, editor: S2, default_contents: S3) -> Result<String>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    // Create named temporary file
    let mut temp_file = NamedTempFile::with_suffix(suffix.as_ref())?;
    temp_file.write_all(default_contents.as_ref().as_bytes())?;
    temp_file.flush()?;

    // Execute editor for editing
    let path = temp_file.path();
    let status = Command::new(editor.as_ref()).arg(path).status()?;
    if !status.success() {
        bail!("editor `{}` did not exit successfully", editor.as_ref());
    }

    // Return file contents
    Ok(fs::read_to_string(path)?)
}

pub fn view_file<S, P>(viewer: S, path: P) -> Result<()>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    // Create temporary file
    let file_name = path
        .as_ref()
        .file_name()
        .ok_or(anyhow!("not a path of file"))?;
    let mut suffix = String::from(".");
    suffix.push_str(&file_name.to_string_lossy());
    let mut temp_file = NamedTempFile::with_suffix(&suffix)?;

    // Copy orginal file
    {
        let mut file = File::open(path)?;
        io::copy(&mut file, &mut temp_file)?;
    }

    // Execute viewer
    let path = temp_file.path();
    let status = Command::new(viewer.as_ref()).arg(path).status()?;
    if !status.success() {
        bail!("viewer `{}` did not exit successfully", viewer.as_ref());
    }

    // Success
    Ok(())
}
