#[macro_export]
macro_rules! normal {
    ($($arg:tt)*) => {{
        println!("{}", console::style(format!($($arg)*)).bright().black());
        Ok(())
    }};
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {{
        println!("{}", console::style(format!($($arg)*)).bold().bright().green());
        Ok(())
    }};
}
