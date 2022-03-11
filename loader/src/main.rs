#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use common::{frame_buffer::*, vec2::Vec2};
use core::{arch::asm, fmt::Write, mem};
use uefi::{
  prelude::*,
  proto::{
    console::gop::GraphicsOutput,
    media::file::{File, FileAttribute, FileMode, FileType::*},
  },
  table::boot::MemoryType,
  ResultExt,
};

#[entry]
fn main(_handle: Handle, mut table: SystemTable<Boot>) -> Status {
  table.stdout().clear().unwrap_success();
  uefi_services::init(&mut table).unwrap_success();
  writeln!(table.stdout(), "[Log] Started boot loader").unwrap();
  // get memmap
  /*
  let memmup_size = table.boot_services().memory_map_size().map_size;
  writeln!(table.stdout(), "size: {}", memmup_size);
  let mut buf = Vec![0; memmup_size];
  let memmap = table.boot_services().memory_map(&mut buf);
  */
  // get GOP
  writeln!(table.stdout(), "[Log] Loading GOP").unwrap();
  let gop = unsafe {
    &mut *(table
      .boot_services()
      .locate_protocol::<GraphicsOutput>()
      .unwrap_success()
      .get())
  };
  let frame_buffer = FrameBuffer {
    frame_buffer: gop.frame_buffer().as_mut_ptr() as *mut [u8; 4],
    stride:       gop.current_mode_info().stride(),
    resolution:   Vec2::<u32> {
      x: gop.current_mode_info().resolution().0 as u32,
      y: gop.current_mode_info().resolution().1 as u32,
    },
    pixel_format: gop.current_mode_info().pixel_format(),
  };

  // open root dir
  writeln!(table.stdout(), "[Log] Loading root dir").unwrap();
  let mut dir = unsafe {
    &mut *table
      .boot_services()
      .get_image_file_system(_handle)
      .unwrap_success()
      .interface
      .get()
  }
  .open_volume()
  .unwrap_success();
  // find kernel file and check file size
  const BUFFER_SIZE: usize = 128;
  let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
  let kernel_size = loop {
    match dir.read_entry(&mut buffer).unwrap_success() {
      Some(file) => {
        if file.file_name().as_string() == "kernel.elf" {
          break file.file_size();
        }
      }
      None => panic!("`kernel.elf` is not found"),
    }
  };
  writeln!(table.stdout(), "[Debug] Kernel size: {} bytes", kernel_size).unwrap();
  // load kernel to pool
  // FileAttribute is invalid in Read-Only open like this
  let kernel_file = match dir
    .open("kernel.elf", FileMode::Read, FileAttribute::READ_ONLY)
    .unwrap_success()
    .into_type()
    .unwrap_success()
  {
    Regular(f) => f,
    _ => panic!("Invalid file type"),
  };

  let loader_pool = table
    .boot_services()
    .allocate_pool(MemoryType::LOADER_DATA, kernel_size as usize)
    .unwrap_success();

  loop {
    unsafe {
      asm!("hlt");
    }
  }
}
