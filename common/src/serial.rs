use uart_16550::SerialPort;

pub fn read() -> u8 {
  let mut serial_port = unsafe { SerialPort::new(0x3f8) };
  serial_port.init();
  serial_port.receive()
}

pub fn print_raw(value: u8) {
  let mut serial_port = unsafe { SerialPort::new(0x3f8) };
  serial_port.init();
  serial_port.send_raw(value);
}

pub fn print(args: core::fmt::Arguments) {
  use core::fmt::Write;

  let mut serial_port = unsafe { SerialPort::new(0x3f8) };
  serial_port.init();
  serial_port.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::print(format_args!($($arg)*));
    };
}
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\r\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\r\n"), $($arg)*));
}
