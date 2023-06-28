#![no_std]
#![no_main]

use apps::syscall;

#[no_mangle]
fn _start() -> ! {
  syscall::exit(0x17)
}
