#![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
  Vertical,
  Horizontal,
}

use core::ops::{Add, Mul, Sub};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2<T> {
  pub x: T,
  pub y: T,
}
impl<T: Add<Output = T>> Add for Vec2<T> {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}
impl<T: Sub<Output = T>> Sub for Vec2<T> {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}
impl<T: Mul<Output = T>> Mul for Vec2<T> {
  type Output = Self;
  fn mul(self, other: Self) -> Self {
    Self {
      x: self.x * other.x,
      y: self.y * other.y,
    }
  }
}

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
    () => ($crate::serial_print!("\r\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\r\n"), $($arg)*));
}
