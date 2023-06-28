#![no_std]
#![no_main]

use apps::syscall;

#[no_mangle]
fn _start() -> u64 {
  syscall::diff(8, 4);
  loop {}
}
