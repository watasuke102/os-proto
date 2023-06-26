#![no_std]
#![no_main]
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

use alloc::{alloc::Layout, string::String, vec::Vec};
use common::{
  log_debug, log_error, log_info, memory_map::MemoryMap, serial, serial_print, serial_println,
};
use core::panic::PanicInfo;
use memory::*;
use x86_64::instructions::hlt;

#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
  log_error!("PANIC: {}", info);
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
  log_info!("Welcome to kernel!");
  segment::init();
  paging::init();
  global_allocator::init(&memmap);
  interrupt::init();
  unsafe {
    log_debug!("writing to Phys 0x4000_0000");
    let p = 0xffff_8000_0000_4000u64 as *mut u64;
    let value = 0xdeadbeef;
    *(0x4000_4000 as *mut u64) = value;
    // *p = value;
    log_debug!("Written: {:p} == 0x{:x}", p, *p);
    if *p != value {
      panic!("User Page isn't written correctly");
    }
  }

  let initfs = fat::Fat::new(initfs_img.to_vec());

  let mut command = String::new();
  log_debug!("Start loop");
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
      } else if c as u8 == 127 && !command.is_empty() {
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
      "" => (),
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
      "int" => x86_64::instructions::interrupts::int3(),
      "dead" => unsafe {
        *((!0xffu64) as *mut u64) = 0;
      },
      _ => serial_println!("Error: Unknown command"),
    })();
    command.clear();
    serial_println!();
  }
}
