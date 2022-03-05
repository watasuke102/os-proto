#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use core::fmt::Write;
use uart_16550::SerialPort;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
  Vertical,
  Horizontal,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
  pub x: T,
  pub y: T,
}

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
    () => ($crate::serial_print!("\r\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\r\n"), $($arg)*));
}
