#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod memory;
mod pixel_writer;
use core::panic::PanicInfo;
use kernel::print;
use kernel::{Direction, Vec2};
use memory::*;
use pixel_writer::*;

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
  kernel::serial_println!("[FATAL] {}", _info);
  loop {}
}

#[no_mangle]
pub extern "sysv64" fn kernel_main(
  config: &mut FrameBufferConfig,
  _memmap: u64,
  _acpi_table: u64,
) -> ! {
  config.write_rect(
    Vec2::<u32> { x: 0, y: 0 },
    config.size(),
    &PixelColor::from_hex(0x32a852),
    true,
  );

  segment::init();
  config.write_rect(
    Vec2::<u32> { x: 0, y: 0 },
    config.size(),
    &PixelColor::from_hex(0x6134eb),
    true,
  );

  paging::init();
  config.write_rect(
    Vec2::<u32> { x: 0, y: 0 },
    config.size(),
    &PixelColor::from_hex(0xeb4034),
    true,
  );

  loop {}
}
