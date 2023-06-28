#![no_std]
#![no_main]

use apps::syscall;

#[no_mangle]
fn _start() -> u64 {
  syscall::add(1, 2, 3, 4, 5, 6);
  loop {}
}
