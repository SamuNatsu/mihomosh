use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::Command,
};

use anyhow::{Context, Result, anyhow, bail};
use tempfile::NamedTempFile;

use crate::{println_secondary, utils::prompt};

const DEFAULT_EDITOR: &str = if cfg!(windows) { "edit" } else { "nano" };

pub fn edit_file<P, S1, S2>(
    path: P,
    editor: Option<S1>,
    default_contents: Option<S2>,
) -> Result<bool>
where
    P: AsRef<Path>,
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    // Check file
    if !path.as_ref().is_file() {
        match default_contents {
            Some(default_contents) => fs::write(&path, default_contents.as_ref())
                .with_context(|| format!("Fail to write file `{}`", path.as_ref().display()))?,
            None => bail!("File `{}` not found", path.as_ref().display()),
        }
    }

    // Get suffix
    let ext = path
        .as_ref()
        .extension()
        .unwrap_or_default()
        .to_string_lossy();
    let mut suffix = String::from(".");
    suffix.push_str(&ext);

    // Edit through temporary file
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Fail to read file `{}`", path.as_ref().display()))?;
    let contents = edit_temp_file(&suffix, editor, &contents).context("Fail to edit temporary")?;

    // Confirm to save
    let input = prompt::confirm("Are you sure to save the changes?")
        .context("Fail to show confirm prompt")?;
    if !input {
        println_secondary!("Changes discarded");
        return Ok(false);
    }

    // Update file
    fs::write(&path, &contents)
        .with_context(|| format!("Fail to write file `{}`", path.as_ref().display()))?;
    Ok(true)
}

pub fn edit_temp_file<S1, S2, S3>(
    suffix: S1,
    editor: Option<S2>,
    default_contents: S3,
) -> Result<String>
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

    // Get editor
    let editor = editor.map_or(
        env::var("EDITOR").unwrap_or(DEFAULT_EDITOR.to_owned()),
        |s| s.as_ref().to_owned(),
    );

    // Execute editor
    let path = temp_file.path();
    let status = Command::new(&editor).arg(path).status().with_context(|| {
        format!(
            "Fail to execute program `{editor}` with argument `{}`",
            path.display()
        )
    })?;
    if !status.success() {
        bail!("Editor `{editor}` exited with status `{status}`");
    }

    // Return file contents
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Fail to read file `{}`", path.display()))?;
    Ok(contents)
}

pub fn view_file<P, S>(path: P, viewer: S) -> Result<bool>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    // Check file
    if !path.as_ref().is_file() {
        return Ok(false);
    }

    // Create temporary file
    let file_name = path.as_ref().file_name().ok_or(anyhow!(
        "Path `{}` is not a file path",
        path.as_ref().display()
    ))?;

    let mut suffix = String::from(".");
    suffix.push_str(&file_name.to_string_lossy());

    let mut temp_file = NamedTempFile::with_suffix(&suffix)
        .with_context(|| format!("Fail to create named temporary file with suffix `{suffix}`"))?;

    // Copy orginal file
    let mut file = File::open(&path)
        .with_context(|| format!("Fail to open file `{}`", path.as_ref().display()))?;
    io::copy(&mut file, &mut temp_file).with_context(|| {
        format!(
            "Fail to copy file from `{}` to `{}`",
            path.as_ref().display(),
            temp_file.path().display()
        )
    })?;
    drop(file);

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
        bail!("Viewer `{}` exited with status `{status}`", viewer.as_ref());
    }

    // Success
    Ok(true)
}
