macro_rules! normal {
    ($($arg:tt)*) => {{
        println!("{}", console::style(format!($($arg)*)).bright().black());
        Ok(())
    }};
}
pub(crate) use normal;

macro_rules! success {
    ($($arg:tt)*) => {{
        println!("{}", console::style(format!($($arg)*)).bold().bright().green());
        Ok(())
    }};
}
pub(crate) use success;

macro_rules! fail {
    ($($arg:tt)*) => {{
        println!("{}", console::style(format!($($arg)*)).bold().bright().red());
        Ok(())
    }};
}
pub(crate) use fail;
