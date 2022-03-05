use core::fmt::Write;
use uart_16550::SerialPort;

pub fn print(args: core::fmt::Arguments) {
  let mut serial_port = unsafe { SerialPort::new(0x3f8) };
  serial_port.init();
  serial_port.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        print(format_args!($($arg)*));
    };
}
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
