#[macro_export]
macro_rules! log {
    ($color:literal, $typ:literal => $($arg:tt)*) => {
        $crate::serial!("\x1b[{}{}\x1b[0m: {}:{}: ", $color, $typ, file!(), line!());
        $crate::serialn!($($arg)*);
    }
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log!("1;33m", "info" => $($arg)*);
    }
}
#[macro_export]
macro_rules! ok {
    ($($arg:tt)*) => {
        $crate::log!("1;32m", "ok"  => $($arg)*);
    }
}
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        $crate::log!("1;31m", "error" => $($arg)*);
    }
}
