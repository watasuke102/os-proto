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
  frame_manager.draw(config);
  serial_println!("[Info] All done!");

  let mut key_buffer = x86_64::instructions::port::PortReadOnly::<u8>::new(0x60);
  let mut key_stat = x86_64::instructions::port::PortReadOnly::<u8>::new(0x64);

  let mut pressed_before = false;
  loop {
    // pooling until input buffer is full (bit 0 is 1)
    while (unsafe { key_stat.read() } & 1) != 1 {}

    let x = unsafe { key_buffer.read() };
    match x {
      // j
      0x24 => {
        if !pressed_before {
          pressed_before = true;
          frame_manager.add(Direction::Bottom, PixelColor::from_hex(0xe8322c));
          crate::memory::global_allocator::print_free_memory();
          frame_manager.draw(config);
        }
      }
      // l
      0x26 => {
        if !pressed_before {
          pressed_before = true;
          frame_manager.add(Direction::Right, PixelColor::from_hex(0x2c8ae8));
          crate::memory::global_allocator::print_free_memory();
          frame_manager.draw(config);
        }
      }
      // c
      0x2e => {
        if !pressed_before {
          pressed_before = true;
          frame_manager.remove_all_frame();
          frame_manager.draw(config);
        }
      }

      _ => pressed_before = false,
    }
  }
}
