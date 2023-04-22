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
mod exec;
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
use core::{arch::asm, mem::transmute, panic::PanicInfo};
use elf_rs::{Elf, ElfFile, ElfType, ProgramType};
use kernel::*;
use memory::*;
use uefi::proto::media::file;
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

    (|| match commands[0] {
      "echo" => {
        for arg in commands.iter().skip(1) {
          serial_print!("{} ", arg);
        }
        serial_println!();
      }
      "ls" => {
        for item in &initfs.files {
          if commands.len() >= 2 && commands[1] == "-l" {
            serial_println!("[0x{:02x}, {:4}] {}", item.attrib, item.size, item.name);
          } else {
            serial_print!("{} ", item.name);
          }
        }
      }
      "cat" => {
        if commands.len() < 2 {
          serial_println!("Error: please specify file name");
        } else {
          let Some(i) = initfs.get_file_index_from_name(commands[1]) else {
            serial_println!("Error: File not found");
            return;
          };
          let data = initfs.data(i);
          for byte in data {
            serial_print!("{}", *byte as char);
          }
        }
      }
      "dump" => {
        if commands.len() < 2 {
          serial_println!("Error: please specify file name");
        } else {
          let Some(i) = initfs.get_file_index_from_name(commands[1]) else {
            serial_println!("Error: File not found");
            return;
          };
          let data = initfs.data(i);
          for byte in data {
            serial_print!("{:02x} ", *byte);
          }
        }
      }
      "exec" => {
        if commands.len() < 2 {
          serial_println!("Error: please specify file name");
        } else {
          let Some(i) = initfs.get_file_index_from_name(commands[1]) else {
            serial_println!("Error: File not found");
            return;
          };
          exec::execute_elf(initfs.data(i), initfs.item_addr(i) as u64);
        }
      }
      _ => serial_println!("Error: Unknown command"),
    })();
    command.clear();
    serial_println!();
  }
}
