#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
pub fn handle_panic(_: &PanicInfo) -> ! {
  loop {}
}

pub fn f() -> u64 {
  128u64
}
