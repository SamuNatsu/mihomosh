use std::{io::BufRead, path::Path, sync::LazyLock};

use anyhow::Result;
use syntect::{
    easy::{HighlightFile, HighlightLines},
    highlighting::ThemeSet,
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(|| SyntaxSet::load_defaults_newlines());
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(|| ThemeSet::load_defaults());

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut highlighter =
        HighlightFile::new(&path, &SYNTAX_SET, &THEME_SET.themes["base16-ocean.dark"])?;

    let mut contents = String::new();
    let mut line = String::new();
    while highlighter.reader.read_line(&mut line)? > 0 {
        let regions = highlighter
            .highlight_lines
            .highlight_line(&line, &SYNTAX_SET)?;
        contents.push_str(&syntect::util::as_24_bit_terminal_escaped(
            &regions[..],
            false,
        ));

        line.clear();
    }

    Ok(contents)
}

pub fn from_string<S1, S2>(extension: S1, contents: S2) -> Result<String>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let syntax = SYNTAX_SET
        .find_syntax_by_extension(extension.as_ref())
        .unwrap_or(SYNTAX_SET.find_syntax_plain_text());
    let mut highlighter = HighlightLines::new(syntax, &THEME_SET.themes["base16-ocean.dark"]);

    let mut buf = String::new();
    for line in LinesWithEndings::from(contents.as_ref()) {
        let regions = highlighter.highlight_line(line, &SYNTAX_SET)?;
        buf.push_str(&syntect::util::as_24_bit_terminal_escaped(
            &regions[..],
            false,
        ));
    }

    Ok(buf)
}
