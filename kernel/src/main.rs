#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]
#![feature(vec_into_raw_parts)]
#![feature(alloc_error_handler)]
#![feature(associated_type_bounds)]

extern crate alloc;
mod interrupt;
mod linked_list;
mod memory;

use alloc::{alloc::Layout, vec};
use common::{memory_map::MemoryMap, serial_print, serial_println};
use core::{arch::asm, panic::PanicInfo};
use kernel::*;
use memory::*;
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
pub extern "sysv64" fn kernel_main(memmap: &MemoryMap) -> ! {
  serial_println!("Welcome to kernel!");
  segment::init();
  paging::init();
  global_allocator::init(&memmap);
  interrupt::init();

  serial_println!("Start loop");
  loop {}
}
