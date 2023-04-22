#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
pub fn handle_panic(_: &PanicInfo) -> ! {
  loop {}
}

macro_rules! exit {
    () => (exit!(0));
    ($exit_code:expr) => (unsafe{
      asm!(
      "mov rdi, {0:r}",
      "ret",
      in(reg) $exit_code,
      );
    });
}

#[no_mangle]
fn _start() -> ! {
  exit!(0x17);
  loop {}
}
