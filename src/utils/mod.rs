pub mod dialog;
pub mod log;
pub mod tempfile;
pub mod tools;

pub trait IntoSizeStr {
    fn into_size_str(self) -> String;
}

impl IntoSizeStr for i64 {
    fn into_size_str(self) -> String {
        match self {
            ..0 => "?".into(),
            x @ ..1_024 => format!("{x}B"), // Within 1 KB
            x @ ..1_048_576 => format!("{:.1}KB", x as f64 / 1_024f64), // Within 1 MB
            x @ ..1_073_741_824 => format!("{:.1}MB", x as f64 / 1_048_576f64), // Within 1 GB
            x => format!("{:.1}GB", x as f64 / 1_073_741_824f64),
        }
    }
}
