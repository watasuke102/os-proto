#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
pub fn handle_panic(_: &PanicInfo) -> ! {
  loop {}
}

fn fib(n: u64) -> u64 {
  match n {
    0 => 0,
    1 => 1,
    _ => fib(n - 2) + fib(n - 1),
  }
}

#[no_mangle]
fn _start() -> u64 {
  fib(16)
}
