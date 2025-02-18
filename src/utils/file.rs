use std::{env, fs, io::Write, path::Path, process::Command};

use anyhow::{bail, Result};
use tempfile::NamedTempFile;

use super::highlight;

pub fn edit_temp_file<S1, S2, S3>(
    suffix: S1,
    editor: Option<S2>,
    default_contents: Option<S3>,
) -> Result<String>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    // Create temporary file
    let mut temp_file = NamedTempFile::with_suffix(suffix.as_ref())?;

    // Set default contents
    if let Some(contents) = default_contents {
        temp_file.write_all(contents.as_ref().as_bytes())?;
        temp_file.flush()?;
    }

    // Edit file
    let path = temp_file.path();
    let editor = editor
        .map(|v| v.as_ref().to_string())
        .unwrap_or(env::var("EDITOR").unwrap_or("vim".into()));

    let status = Command::new(&editor).arg(path).status()?;
    if !status.success() {
        bail!("editor `{}` did not exit successfully", editor);
    }

    // Success
    Ok(fs::read_to_string(path)?)
}

pub fn show_file<P: AsRef<Path>>(path: P) -> Result<()> {
    // Highlight contents
    let contents = highlight::from_file(&path)?;

    // Show contents
    let name = path
        .as_ref()
        .file_name()
        .map_or("", |v| v.to_str().unwrap_or(""));
    show_contents(&name, &contents, false)?;

    // Success
    Ok(())
}

pub fn show_contents<S1, S2>(suffix: S1, contents: S2, highlight: bool) -> Result<()>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    // Create temporary file
    let mut temp_file = NamedTempFile::with_suffix(format!("-{}", suffix.as_ref()))?;

    // Set contents
    if highlight {
        let extension = Path::new(suffix.as_ref())
            .extension()
            .map_or("", |v| v.to_str().unwrap_or(""));
        let contents = highlight::from_string(extension, contents.as_ref())?;
        temp_file.write_all(contents.as_bytes())?;
        temp_file.flush()?;
    } else {
        temp_file.write_all(contents.as_ref().as_bytes())?;
        temp_file.flush()?;
    }

    // Show contents
    let path = temp_file.path();
    let status = Command::new("less").arg("-R").arg(path).status()?;
    if !status.success() {
        bail!("`less` did not exit successfully");
    }

    // Success
    Ok(())
}
