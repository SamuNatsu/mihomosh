#[macro_export]
macro_rules! style_fmt {
    ($($arg:tt)*) => {
       console::style(format!($($arg)*))
    };
}

// Predefined formatted styled string
#[macro_export]
macro_rules! str_primary {
    ($($arg:tt)*) => {
        $crate::style_fmt!($($arg)*).bold().bright().blue().to_string()
    };
}

#[macro_export]
macro_rules! str_secondary {
    ($($arg:tt)*) => {
        $crate::style_fmt!($($arg)*).italic().bright().black().to_string()
    };
}

#[macro_export]
macro_rules! str_success {
    ($($arg:tt)*) => {
        $crate::style_fmt!($($arg)*).bold().bright().green().to_string()
    };
}

#[macro_export]
macro_rules! str_warn {
    ($($arg:tt)*) => {
        $crate::style_fmt!($($arg)*).bold().bright().yellow().to_string()
    };
}

#[macro_export]
macro_rules! str_help {
    ($($arg:tt)*) => {
        $crate::style_fmt!($($arg)*).underlined().cyan().to_string()
    };
}

#[macro_export]
macro_rules! str_danger {
    ($($arg:tt)*) => {
        $crate::style_fmt!($($arg)*).bold().bright().red().to_string()
    };
}

#[macro_export]
macro_rules! str_contrast {
    ($($arg:tt)*) => {
        $crate::style_fmt!($($arg)*).bold().bright().black().on_white().to_string()
    };
}

// Print line predefined formatted styled string
#[macro_export]
macro_rules! println_primary {
    ($($arg:tt)*) => {
        println!("{}", $crate::str_primary!($($arg)*))
    };
}

#[macro_export]
macro_rules! println_secondary {
    ($($arg:tt)*) => {
        println!("{}", $crate::str_secondary!($($arg)*))
    };
}

#[macro_export]
macro_rules! println_success {
    ($($arg:tt)*) => {
        println!("{}", $crate::str_success!($($arg)*))
    };
}

#[macro_export]
macro_rules! println_warn {
    ($($arg:tt)*) => {
        println!("{}", $crate::str_warn!($($arg)*))
    };
}

#[macro_export]
macro_rules! println_help {
    ($($arg:tt)*) => {
        println!("{}", $crate::str_help!($($arg)*))
    };
}

#[macro_export]
macro_rules! println_danger {
    ($($arg:tt)*) => {
        eprintln!("{}", $crate::str_danger!($($arg)*))
    };
}

#[macro_export]
macro_rules! println_contrast {
    ($($arg:tt)*) => {
        println!("{}", $crate::str_contrast!($($arg)*))
    };
}
