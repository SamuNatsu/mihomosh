use std::io::{self, Write};

use eyre::{Context, Result};
use owo_colors::OwoColorize;

pub fn alert<S: AsRef<str>>(msg: S) {
    println!("{}", format!("# {}", msg.as_ref()).bold().bright_red());
}

pub fn prompt<S: AsRef<str>>(msg: S) -> Result<String> {
    // Print prompt
    print!(
        "{} {} ",
        "?".bold().bright_green(),
        msg.as_ref().bold().bright_white()
    );
    io::stdout().flush()?;

    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Return input
    Ok(input.trim().to_owned())
}

pub fn confirm<S: AsRef<str>>(msg: S, default: bool) -> Result<bool> {
    let hint = if default { "[Y/n]" } else { "[y/N]" };
    let msg = format!("{} {hint}", msg.as_ref());

    // Check loop
    loop {
        let input = prompt(&msg)
            .wrap_err("failed to show prompt dialog")?
            .to_lowercase();

        // Return default
        if input.is_empty() {
            return Ok(default);
        }

        // Parse input
        match input.as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => alert("Invalid input. Please type 'y' for yes or 'n' for no."),
        }
    }
}
