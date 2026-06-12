macro_rules! primary {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        println!("{}", format!($($arg)*).bold().bright_purple())
    }};
}
pub(crate) use primary;

macro_rules! log {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        println!("{}", format!($($arg)*).bright_black())
    }};
}
pub(crate) use log;

macro_rules! info {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        println!("{}", format!($($arg)*).bright_blue())
    }};
}
pub(crate) use info;

macro_rules! success {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        println!("{}", format!($($arg)*).bright_green())
    }};
}
pub(crate) use success;

macro_rules! error {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        println!("{}", format!($($arg)*).bright_red())
    }};
}
pub(crate) use error;
