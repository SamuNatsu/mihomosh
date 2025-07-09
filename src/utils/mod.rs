pub mod api;
pub mod file;
pub mod macros;
pub mod path;

use std::io::{self, Write};

use anyhow::Result;
use rand::{TryRngCore, rngs::OsRng};

use crate::str_primary;

pub fn prompt<S: AsRef<str>>(prompt: S) -> Result<String> {
    print!("{}", str_primary!("{}", prompt.as_ref()));
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_owned())
}

pub fn gen_uuid() -> Result<String> {
    let mut rng = OsRng;
    let mut buf = vec![0u8; 4];
    rng.try_fill_bytes(&mut buf)?;
    Ok(hex::encode(buf))
}
