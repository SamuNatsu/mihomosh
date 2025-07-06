#[macro_export]
macro_rules! style_fmt {
    ($($arg:tt)*) => {
       console::style(format!($($arg)*))
    };
}

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

#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => {{
        println!("{}", console::style(format!($($arg)*)).bold().bright().red());
        Ok(())
    }};
}
