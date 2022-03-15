#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(associated_type_bounds)]

extern crate alloc;
mod interrupt;
mod linked_list;
mod memory;
mod window;

use alloc::{alloc::Layout, vec};
use common::{frame_buffer::*, memory_map::MemoryMap, serial_print, serial_println};
use core::{arch::asm, panic::PanicInfo};
use kernel::*;
use memory::*;
use window::*;
use x86_64::instructions::hlt;

#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
  serial_println!("[PANIC] {}", info);
  loop {
    hlt();
  }
}
#[alloc_error_handler]
fn handle_alloc_error(layout: Layout) -> ! {
  panic!("allocation failed ({:?})", layout);
}

#[no_mangle]
pub extern "sysv64" fn kernel_main(config: &mut FrameBuffer, memmap: &MemoryMap) -> ! {
  serial_println!("Welcome to kernel!");
  segment::init();
  paging::init();
  global_allocator::init(&memmap);
  interrupt::init();

  let mut frame_manager = FrameManager::new(config);
  {
    use Direction::*;
    frame_manager.add(Right, PixelColor::from_hex(0x6134eb));
    frame_manager.add(Right, PixelColor::from_hex(0x34a1eb));
    frame_manager.add(Bottom, PixelColor::from_hex(0xde771d));
    frame_manager.add(Right, PixelColor::from_hex(0xeb4034));
    frame_manager.add(Bottom, PixelColor::from_hex(0x1d2ade));
    frame_manager.add(Right, PixelColor::from_hex(0xde1d74));
  }
  frame_manager.draw(config);

  //x86_64::instructions::interrupts::int3();

  serial_println!("[Info] All done!");
  loop {
    /*
    serial_print!(".");
    for i in 0..40000_000 {}
    x86_64::instructions::interrupts::int3();
    */
    hlt();
  }
}
