#[macro_export]
macro_rules! log {
    ($color:literal, $typ:literal => $($arg:tt)*) => {
        serial!("\x1b[{}{}\x1b[0m: {}:{}: ", $color, $typ, file!(), line!());
        serialn!($($arg)*);
    }
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log!("1;33m", "info" => $($arg)*);
    }
}
#[macro_export]
macro_rules! ok {
    ($($arg:tt)*) => {
        log!("1;32m", "success"  => $($arg)*);
    }
}
