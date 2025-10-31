use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::Command,
};

use anyhow::{Context, Result, anyhow, bail};
use tempfile::NamedTempFile;

use crate::{println_secondary, utils::prompt};

pub fn edit_file<P, S1, S2>(path: P, editor: S1, default_contents: Option<S2>) -> Result<bool>
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
    let contents = edit_temp_file(&suffix, &editor, &contents).with_context(|| {
        format!(
            "Fail to edit temporary file with editor `{}`",
            editor.as_ref()
        )
    })?;

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
    let run_editor = |editor_name: &str| {
        Command::new(editor_name)
            .arg(path)
            .status()
            .with_context(|| {
                format!(
                    "Fail to execute program `{}` with argument `{}`",
                    editor_name,
                    path.display()
                )
            })
    };
    const DEFAULT_EDITOR: &str = "nano";
    // todo: If the cli input is nano (such as mihomosh config edit -e nano),
    // the env EDITOR should not be used.
    // This needs to check whether nano in the command line parameter is passed in by the user.
    // Maybe we should change the command line parameter to Option type
    let status = if editor.as_ref() != DEFAULT_EDITOR {
        run_editor(editor.as_ref())?
    } else {
        match std::env::var("EDITOR") {
            Ok(env_editor) => run_editor(&*env_editor).or_else(|err| {
                if env_editor != DEFAULT_EDITOR {
                    println_secondary!(
                        "Environment editor `{}` exited with status {}, fail back to {}",
                        env_editor,
                        err,
                        DEFAULT_EDITOR
                    );
                    run_editor(DEFAULT_EDITOR)
                } else {
                    Err(err)
                }
            })?,
            // EDITOR environment variable not set, use DEFAULT_EDITOR
            Err(_) => run_editor(DEFAULT_EDITOR)?,
        }
    };

    if !status.success() {
        bail!("Editor `{}` exited with status `{status}`", editor.as_ref());
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
