#![no_std]
#![no_main]

#[allow(unused_imports)]
use apps::syscall;

#[no_mangle]
fn _start() -> ! {
  syscall::add(1, 10, 100, 1000, 10000, 100000);
  syscall::diff(512, 256);
  syscall::exit(0)
}
