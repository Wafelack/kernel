#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        serial!("\x1b[1;33m{}:{}\x1b[0m ", file!(), line!());
        serialn!($($arg)*);
    }
}
