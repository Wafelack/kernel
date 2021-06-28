#[macro_export]
macro_rules! log {
    ($color:literal => $($arg:tt)*) => {
        serial!("\x1b[{}{}:{}\x1b[0m ", $color, file!(), line!());
        serialn!($($arg)*);
    }
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log!("1;33m" => $($arg)*);
    }
}
#[macro_export]
macro_rules! ok {
    ($($arg:tt)*) => {
        log!("1;32m" => $($arg)*);
    }
}
