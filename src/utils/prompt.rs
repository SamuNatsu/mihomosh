use std::io::{self, Write};

use anyhow::{Context, Result};

use crate::str_primary;

fn prompt<S: AsRef<str>>(msg: S) -> Result<String> {
    print!("{}", str_primary!("{}", msg.as_ref()));
    io::stdout().flush().context("Fail to flush STDOUT")?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Fail to read line from STDIN")?;

    Ok(input.trim().to_owned())
}

pub fn confirm<S: AsRef<str>>(msg: S) -> Result<bool> {
    let input = prompt(format!("{} (y/N) ", msg.as_ref()))
        .with_context(|| format!("Fail to show prompt with message `{}`", msg.as_ref()))?;
    Ok(input.to_lowercase() == "y")
}
