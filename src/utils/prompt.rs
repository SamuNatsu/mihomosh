use std::io::{self, BufRead, Write};

use anyhow::Result;
use regex::Regex;

pub fn ask<S: AsRef<str>>(prompt: S) -> Result<String> {
    // Print prompt
    print!("{}", prompt.as_ref());
    io::stdout().lock().flush()?;

    // Read input
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;

    // Return result
    Ok(input)
}

pub fn confirm<S: AsRef<str>>(prompt: S) -> Result<bool> {
    // Get input
    let input = ask(console::style(format!("{} (y/N): ", prompt.as_ref()))
        .bold()
        .bright()
        .yellow()
        .to_string())?;

    // Check input
    let re = Regex::new(r"^[yY]$")?;
    Ok(re.is_match(&input.trim()))
}
