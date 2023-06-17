use crate::serial_println;

pub fn print_log(prefix: &'static str, args: core::fmt::Arguments) {
  serial_println!("[{:5}] {}", prefix, args);
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logger::print_log("Debug", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logger::print_log("Info", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::logger::print_log("Warn", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logger::print_log("Error", format_args!($($arg)*));
    };
}
