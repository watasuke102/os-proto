#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]
#![feature(associated_type_bounds)]

extern crate alloc;
mod linked_list;
mod memory;
mod pixel_writer;
mod window;

use alloc::{alloc::Layout, vec};
use core::{arch::asm, panic::PanicInfo};
use kernel::*;
use kernel::{Direction, Vec2};
use memory::*;
use pixel_writer::*;
use window::*;

use crate::memory::memory_map::MemoryType;

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
  segment::init();
  paging::init();
  global_allocator::init(&memmap);

  let mut frame_manager = FrameManager::new(config);
  let dir = Direction::Horizontal;
  frame_manager.add(dir, PixelColor::from_hex(0x6134eb));
  frame_manager.add(dir, PixelColor::from_hex(0x34a1eb));
  frame_manager.add(Direction::Vertical, PixelColor::from_hex(0xde771d));
  frame_manager.add(dir, PixelColor::from_hex(0xeb4034));
  frame_manager.draw(config);

  loop {
    unsafe {
      asm!("hlt");
    }
  }
}
