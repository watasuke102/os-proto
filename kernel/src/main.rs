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
mod fat;
mod interrupt;
mod linked_list;
mod memory;

use alloc::{
  alloc::Layout,
  string::String,
  vec::{self, Vec},
};
use common::{memory_map::MemoryMap, serial, serial_print, serial_println};
use core::{arch::asm, panic::PanicInfo};
use kernel::*;
use memory::*;
use x86_64::instructions::hlt;

use crate::interrupt::init;

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
pub extern "sysv64" fn kernel_main(memmap: &MemoryMap, initfs_img: &Vec<u8>) -> ! {
  serial_println!("Welcome to kernel!");
  segment::init();
  paging::init();
  global_allocator::init(&memmap);
  interrupt::init();

  let initfs = fat::Fat::new(initfs_img.to_vec());

  let mut command = String::new();
  serial_println!("Start loop");
  loop {
    serial_print!(">> ");
    loop {
      let c = serial::read() as char;
      if c >= ' ' && c <= '~' {
        serial_print!("{}", c);
        command.push(c);
      } else if c as u8 == 13 {
        // Enter
        break;
      } else if c as u8 == 127 {
        // BS
        serial::print_raw(8);
        serial::print_raw(' ' as u8);
        serial::print_raw(8);
        command.pop();
      }
    }
    serial_println!();

    let commands: Vec<&str> = command.split(' ').collect();

    match commands[0] {
      "echo" => {
        for arg in commands.iter().skip(1) {
          serial_print!("{} ", arg);
        }
        serial_println!();
      }
      "ls" => {
        for item in &initfs.files {
          if commands.len() >= 2 && commands[1] == "-l" {
            serial_println!("{} [{:02x}, {}]", item.name, item.attrib, item.size);
          } else {
            serial_print!("{} ", item.name);
          }
        }
        serial_println!();
      }
      "cat" => {
        if commands.len() < 2 {
          serial_println!("Error: please specify file name");
        } else {
          for (i, item) in initfs.files.iter().enumerate() {
            if item.name.as_str() != commands[1] {
              continue;
            }
            let data = initfs.data(i);
            for byte in data {
              serial_print!("{}", *byte as char);
            }
          }
        }
      }
      _ => serial_println!("Unknown command"),
    }
    command.clear();
  }
}
