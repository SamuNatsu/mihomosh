use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::Command,
};

use anyhow::{Context, Result, anyhow, bail};
use tempfile::NamedTempFile;

pub fn edit_temp_file<S1, S2, S3>(suffix: S1, editor: S2, default_contents: S3) -> Result<String>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    // Create named temporary file
    let mut temp_file = NamedTempFile::with_suffix(suffix.as_ref()).with_context(|| {
        format!(
            "Fail to create named temporary file with suffix `{}`",
            suffix.as_ref()
        )
    })?;
    temp_file
        .write_all(default_contents.as_ref().as_bytes())
        .with_context(|| format!("Fail to write file `{}`", temp_file.path().display()))?;
    temp_file
        .flush()
        .with_context(|| format!("Fail to flush file `{}`", temp_file.path().display()))?;

    // Execute editor
    let path = temp_file.path();
    let status = Command::new(editor.as_ref())
        .arg(path)
        .status()
        .with_context(|| {
            format!(
                "Fail to execute program `{}` with argument `{}`",
                editor.as_ref(),
                path.display()
            )
        })?;
    if !status.success() {
        bail!(
            "Editor `{}` exited with status `{}`",
            editor.as_ref(),
            status
        );
    }

    // Return file contents
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Fail to read file `{}`", path.display()))?;
    Ok(contents)
}

pub fn view_file<S, P>(viewer: S, path: P) -> Result<()>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    // Create temporary file
    let file_name = path.as_ref().file_name().ok_or(anyhow!(
        "Path `{}` is not a file path",
        path.as_ref().display()
    ))?;

    let mut suffix = String::from(".");
    suffix.push_str(&file_name.to_string_lossy());

    let mut temp_file = NamedTempFile::with_suffix(&suffix).with_context(|| {
        format!(
            "Fail to create named temporary file with suffix `{}`",
            suffix
        )
    })?;

    // Copy orginal file
    {
        let mut file = File::open(&path)
            .with_context(|| format!("Fail to open file `{}`", path.as_ref().display()))?;
        io::copy(&mut file, &mut temp_file).with_context(|| {
            format!(
                "Fail to copy file from `{}` to `{}`",
                path.as_ref().display(),
                temp_file.path().display()
            )
        })?;
    }

    // Execute viewer
    let path = temp_file.path();
    let status = Command::new(viewer.as_ref())
        .arg(path)
        .status()
        .with_context(|| {
            format!(
                "Fail to execute program `{}` with argument `{}`",
                viewer.as_ref(),
                path.display()
            )
        })?;
    if !status.success() {
        bail!(
            "Viewer `{}` exited with status `{}`",
            viewer.as_ref(),
            status
        );
    }

    // Success
    Ok(())
}
