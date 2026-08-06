mod ansi;
mod history;
mod terminal;

pub use ansi::*;
pub use terminal::*;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        #[allow(clippy::used_underscore_items)]
        $crate::driver::console::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", format_args!($($arg)*));
    }};
}
