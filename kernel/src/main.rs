#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]

extern crate alloc;
mod linked_list;
mod memory;
mod pixel_writer;

use alloc::{alloc::Layout, vec};
use core::{arch::asm, panic::PanicInfo};
use kernel::*;
use kernel::{Direction, Vec2};
use memory::*;
use pixel_writer::*;

#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
  kernel::serial_println!("[PANIC] {}", info);
  loop {}
}
#[alloc_error_handler]
fn handle_alloc_error(layout: Layout) -> ! {
  panic!("allocation failed ({:?})", layout);
}

#[no_mangle]
pub extern "sysv64" fn kernel_main(
  config: &mut FrameBufferConfig,
  memmap: &memory_map::MemoryMap,
  _acpi_table: u64,
) -> ! {
  serial_println!("{:?}", memmap);
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

  global_allocator::init(&memmap);
  config.write_rect(
    Vec2::<u32> { x: 0, y: 0 },
    config.size(),
    &PixelColor::from_hex(0x34a1eb),
    true,
  );

  let a = vec![1, 2, 3];
  for e in a {
    serial_print!("{}, ", e);
  }
  serial_println!("\nAll done!!");

  loop {
    unsafe {
      asm!("hlt");
    }
  }
}
