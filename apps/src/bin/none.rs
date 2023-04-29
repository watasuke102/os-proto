#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
pub fn handle_panic(_: &PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
fn _start() -> u64 {
  0x17
}
