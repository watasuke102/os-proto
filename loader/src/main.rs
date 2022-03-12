#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate alloc;
use alloc::{vec, vec::Vec};
use common::{
  frame_buffer::*,
  memory_map::{is_available_memory, MemoryMap},
  vec2::Vec2,
};
use core::{
  arch::asm,
  cmp::{max, min},
  fmt::Write,
  mem::{self, size_of},
};
use elf_rs::{Elf, ElfFile, ProgramType};
use uefi::{
  prelude::*,
  proto::{
    console::gop::GraphicsOutput,
    media::file::{File, FileAttribute, FileMode, FileType::*, RegularFile},
  },
  table::boot::{AllocateType, MemoryAttribute, MemoryType},
  ResultExt,
};

#[entry]
fn main(handle: Handle, mut table: SystemTable<Boot>) -> Status {
  table.stdout().clear().unwrap_success();
  uefi_services::init(&mut table).unwrap_success();
  writeln!(table.stdout(), "[Log] Started boot loader").unwrap();

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

  // open kernel file
  writeln!(table.stdout(), "[Log] Loading kernel").unwrap();
  let (mut kernel_file, kernel_size) = open_file(table.boot_services(), &handle, "kernel.elf");
  let mut loader_pool = vec![0; kernel_size as usize];
  kernel_file.read(&mut loader_pool).unwrap_success();
  writeln!(table.stdout(), "[Debug] {}", loader_pool.len()).unwrap();
  // calculate LOAD segment range
  let elf = Elf::from_bytes(&loader_pool).unwrap();
  let mut kernel_addr_first = Vec::<u64>::new();
  let mut kernel_addr_last = Vec::<u64>::new();
  for program_header in elf.program_header_iter() {
    if program_header.ph_type() == ProgramType::LOAD {
      kernel_addr_first.push(program_header.paddr());
      kernel_addr_last.push(program_header.paddr() + program_header.memsz());
      writeln!(table.stdout(), "[Debug] {:?}", program_header).unwrap();
    }
  }
  // allocate page
  let kernel_page = {
    let kernel_addr_first = *kernel_addr_first.iter().min().unwrap();
    let kernel_addr_last = *kernel_addr_last.iter().max().unwrap();
    writeln!(
      table.stdout(),
      "[Debug] begin: {}, last: {}",
      kernel_addr_first,
      kernel_addr_last
    )
    .unwrap();
    let page_count: usize = ((kernel_addr_last - kernel_addr_first + 0xfff) / 0x1000) as usize;

    table
      .boot_services()
      .allocate_pages(
        AllocateType::Address(kernel_addr_first as usize),
        MemoryType::LOADER_DATA,
        page_count,
      )
      .unwrap_success()
  };
  writeln!(table.stdout(), "[Debug] allocated: {}", kernel_page).unwrap();
  // load kernel to memory

  // get memmap
  let memmap_size = table.boot_services().memory_map_size().map_size;
  let mut memmap = MemoryMap {
    list: Vec::new(),
    len:  0,
  };
  let mut buf = [0 as u8; 1024 * 4];
  let (memmap_key, memmap_iter) = table.boot_services().memory_map(&mut buf).unwrap_success();

  for desc in memmap_iter {
    if is_available_memory(desc.ty) {
      memmap.list.push(*desc);
      memmap.len += 1;
    }
  }

  loop {
    unsafe {
      asm!("hlt");
    }
  }
}

/// Open file and return (file: RegularFile, size: u64)
/// Cause panic when try to open directory (specify directory name)
fn open_file(boot_services: &BootServices, handle: &Handle, name: &str) -> (RegularFile, u64) {
  // open root dir
  let mut dir = unsafe {
    &mut *boot_services
      .get_image_file_system(*handle)
      .unwrap_success()
      .interface
      .get()
  }
  .open_volume()
  .unwrap_success();

  // find file and check file size
  const BUFFER_SIZE: usize = 128;
  let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
  let size = loop {
    match dir.read_entry(&mut buffer).unwrap_success() {
      Some(file) => {
        if file.file_name().as_string() == name {
          break file.file_size();
        }
      }
      None => panic!("`kernel.elf` is not found"),
    }
  };
  // load kernel to pool
  // FileAttribute is invalid in Read-Only open like this
  match dir
    .open("kernel.elf", FileMode::Read, FileAttribute::READ_ONLY)
    .unwrap_success()
    .into_type()
    .unwrap_success()
  {
    Regular(file) => (file, size),
    _ => panic!("Invalid file type"),
  }
}
