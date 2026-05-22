macro_rules! log {
    ($fmt:expr $(, $args:tt)*) => {{
        use owo_colors::OwoColorize;
        println!("{}", format!($fmt $(, $args)*).bright_black())
    }};
}
pub(crate) use log;

macro_rules! success {
    ($fmt:expr $(, $args:tt)*) => {{
        use owo_colors::OwoColorize;
        println!("{}", format!($fmt $(, $args)*).bright_green())
    }};
}
pub(crate) use success;
