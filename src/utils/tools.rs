use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::{Command, ExitStatus},
    sync::OnceLock,
};

use eyre::{Context, Result, bail, eyre};
use tempfile::NamedTempFile;

pub fn view_contents<S1, S2>(contents: S1, suffix: S2) -> Result<()>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let temp_file = create_temp_file_from_contents(contents, suffix)?;
    let viewer = get_viewer();
    let status = run_tool(viewer, temp_file.path())?;

    // Check status
    if status.success() {
        Ok(())
    } else {
        bail!("viewer `{viewer}` exited with status `{status}`");
    }
}

pub fn edit_contents<S1, S2>(contents: S1, suffix: S2) -> Result<String>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let temp_file = create_temp_file_from_contents(contents, suffix)?;
    let editor = get_editor();
    let status = run_tool(editor, temp_file.path())?;

    // Checck status
    if !status.success() {
        bail!("editor `{editor}` exited with status `{status}`");
    }

    // Read new contents
    fs::read_to_string(temp_file.path())
        .wrap_err_with(|| format!("fail to read file `{}`", temp_file.path().display()))
}

fn get_viewer() -> &'static String {
    static INSTANCE: OnceLock<String> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        env::var("PAGER").unwrap_or(if cfg!(windows) {
            "more.com".to_owned()
        } else {
            "less".to_owned()
        })
    })
}

fn get_editor() -> &'static String {
    static INSTANCE: OnceLock<String> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        env::var("VISUAL")
            .or_else(|_| env::var("EDITOR"))
            .unwrap_or(if cfg!(windows) {
                "edit.exe".to_owned()
            } else {
                "vim".to_owned()
            })
    })
}

fn create_temp_file_from_contents<S1, S2>(contents: S1, suffix: S2) -> Result<NamedTempFile>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    // Create temporary file
    let mut temp_file =
        NamedTempFile::with_suffix(format!(".{}", suffix.as_ref())).wrap_err_with(|| {
            format!(
                "fail to create named temporary file with suffix `.{}`",
                suffix.as_ref()
            )
        })?;

    // Copy contents
    temp_file
        .write_all(contents.as_ref().as_bytes())
        .wrap_err_with(|| format!("fail to write file `{}`", temp_file.path().display()))?;

    // Done
    Ok(temp_file)
}

fn create_temp_file_from_path<P: AsRef<Path>>(path: P) -> Result<NamedTempFile> {
    // Create temporary file
    let file_name = path
        .as_ref()
        .file_name()
        .ok_or_else(|| {
            eyre!(
                "fail to retrieve file name from path `{}`",
                path.as_ref().display()
            )
        })?
        .to_string_lossy();
    let mut temp_file =
        NamedTempFile::with_suffix(format!(".{file_name}")).wrap_err_with(|| {
            format!("fail to create named temporary file with suffix `.{file_name}`")
        })?;

    // Copy contents
    let mut origin_file = File::open(&path)
        .wrap_err_with(|| format!("fail to open file `{}`", path.as_ref().display()))?;
    io::copy(&mut origin_file, &mut temp_file).wrap_err_with(|| {
        format!(
            "fail to copy file contents from `{}` to `{}`",
            path.as_ref().display(),
            temp_file.path().display()
        )
    })?;

    // Done
    Ok(temp_file)
}

#[inline]
fn run_tool<S, P>(program: S, path: P) -> Result<ExitStatus>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    Command::new(program.as_ref())
        .arg(path.as_ref())
        .status()
        .wrap_err_with(|| {
            format!(
                "fail to execute `{}` with path `{}`",
                get_viewer(),
                path.as_ref().display()
            )
        })
}
