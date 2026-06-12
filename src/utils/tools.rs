use std::{env, path::Path, process::ExitStatus, sync::LazyLock};

use eyre::{Context, Result, bail, eyre};
use tokio::{
    fs::{self, File},
    io::{self, AsyncWriteExt},
    process::Command,
};

use crate::utils::tempfile::TempFile;

static VIEWER: LazyLock<String> = LazyLock::new(|| {
    const FALLBACK: &str = if cfg!(windows) { "more.com" } else { "less" };

    env::var("PAGER").unwrap_or(FALLBACK.to_owned())
});

static EDITOR: LazyLock<String> = LazyLock::new(|| {
    const FALLBACK: &str = if cfg!(windows) { "edit.exe" } else { "vim" };

    env::var("VISUAL")
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or(FALLBACK.to_owned())
});

pub async fn view_contents<S1, S2>(contents: S1, suffix: S2) -> Result<()>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let temp_file = create_temp_file_from_contents(contents, suffix).await?;
    let status = run_tool(&*VIEWER, temp_file.path()).await?;

    // Check status
    if status.success() {
        Ok(())
    } else {
        bail!("viewer `{}` exited with status `{status}`", *VIEWER);
    }
}

pub async fn view_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let temp_file = create_temp_file_from_path(path).await?;
    let status = run_tool(&*VIEWER, temp_file.path()).await?;

    // Check status
    if status.success() {
        Ok(())
    } else {
        bail!("viewer `{}` exited with status `{status}`", *VIEWER);
    }
}

pub async fn edit_contents<S1, S2>(contents: S1, suffix: S2) -> Result<String>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let temp_file = create_temp_file_from_contents(contents, suffix).await?;
    let status = run_tool(&*EDITOR, temp_file.path()).await?;

    // Check status
    if !status.success() {
        bail!("editor `{}` exited with status `{status}`", *EDITOR);
    }

    // Read new contents
    fs::read_to_string(&temp_file.path())
        .await
        .wrap_err_with(|| format!("failed to read file `{}`", temp_file.path().display()))
}

pub async fn edit_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let temp_file = create_temp_file_from_path(path).await?;
    let status = run_tool(&*EDITOR, temp_file.path()).await?;

    // Check status
    if !status.success() {
        bail!("editor `{}` exited with status `{status}`", *EDITOR);
    }

    // Read new contents
    fs::read_to_string(&temp_file.path())
        .await
        .wrap_err_with(|| format!("failed to read file `{}`", temp_file.path().display()))
}

async fn create_temp_file_from_contents<S1, S2>(contents: S1, suffix: S2) -> Result<TempFile>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    // Create temporary file
    let mut temp_file = TempFile::create(suffix.as_ref()).await?;

    // Copy contents
    temp_file
        .write_all(contents.as_ref().as_bytes())
        .await
        .wrap_err_with(|| format!("failed to write file `{}`", temp_file.path().display()))?;

    // Done
    Ok(temp_file)
}

async fn create_temp_file_from_path<P: AsRef<Path>>(path: P) -> Result<TempFile> {
    // Create temporary file
    let file_name = path
        .as_ref()
        .file_name()
        .ok_or_else(|| {
            eyre!(
                "failed to retrieve file name from path `{}`",
                path.as_ref().display()
            )
        })?
        .to_string_lossy();
    let mut temp_file = TempFile::create(file_name.to_string()).await?;

    // Copy contents
    let mut origin_file = File::open(&path)
        .await
        .wrap_err_with(|| format!("failed to open file `{}`", path.as_ref().display()))?;
    io::copy(&mut origin_file, &mut *temp_file)
        .await
        .wrap_err_with(|| {
            format!(
                "failed to copy file contents from `{}` to `{}`",
                path.as_ref().display(),
                temp_file.path().display()
            )
        })?;

    // Done
    Ok(temp_file)
}

async fn run_tool<S, P>(program: S, path: P) -> Result<ExitStatus>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    Command::new(program.as_ref())
        .arg(path.as_ref())
        .status()
        .await
        .wrap_err_with(|| {
            format!(
                "failed to execute `{}` with path `{}`",
                program.as_ref(),
                path.as_ref().display()
            )
        })
}
