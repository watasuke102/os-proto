#![no_std]

#![no_main]
#![feature(abi_efiapi)]
#![feature(vec_into_raw_parts)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate alloc;
use alloc::{vec, vec::Vec};
use common::{
  frame_buffer::*,
  memory_map::{is_available_memory, MemoryMap, MEMORYMAP_LIST_LEN},
  serial_print, serial_println,
  vec2::Vec2,
};
use core::{arch::asm, fmt::Write};
use elf_rs::{Elf, ElfFile, ProgramType};
use uefi::{
  prelude::*,
  proto::{
    console::gop::GraphicsOutput,
    media::file::{File, FileAttribute, FileMode, FileType::*, RegularFile},
  },
  table::boot::{AllocateType, MemoryDescriptor, MemoryType},
  ResultExt,
  
};

#[derive(Debug)]
struct LoadSegment {
  begin:       u64,
  end:         u64,
  offset:      u64,
  vaddr:       u64,
  memory_size: u64,
  file_size:   u64,
}

#[entry]
fn main(handle: Handle, mut table: SystemTable<Boot>) -> Status {
  macro_rules! print {
    ($($arg:tt)*) => { write!(table.stdout(), "{}", format_args!($($arg)*)).unwrap() };
  }
  macro_rules! println {
    () => { print!("\n"); };
    ($($arg:tt)*) => { print!("{}\n", format_args!($($arg)*)) };
  }

  table.stdout().clear().unwrap_success();
  uefi_services::init(&mut table).unwrap_success();
  println!("[Log] Started boot loader");

  // get GOP
  println!("[Log] Loading GOP");
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
  println!("[Log] Loading kernel");
  let (mut kernel_file, kernel_size) = open_file(table.boot_services(), &handle, "kernel.elf");
  let mut loader_pool = vec![0; kernel_size as usize];
  kernel_file.read(&mut loader_pool).unwrap_success();
  println!("[Debug] {}", loader_pool.len());
  // calculate LOAD segment range
  let elf = Elf::from_bytes(&loader_pool).unwrap();
  let kernel_entry = elf.entry_point();
  println!("[Debug] entrypoint: 0x{:x}", kernel_entry);
  let mut load_segment = Vec::<LoadSegment>::new();
  for program_header in elf.program_header_iter() {
    if program_header.ph_type() == ProgramType::LOAD {
      load_segment.push(LoadSegment {
        begin:       program_header.paddr(),
        end:         program_header.paddr() + program_header.memsz(),
        offset:      program_header.offset(),
        vaddr:       program_header.vaddr(),
        file_size:   program_header.filesz(),
        memory_size: program_header.memsz(),
      });
      println!("{:?}", load_segment[load_segment.len() - 1]);
    }
  }
  // allocate page
  let kernel_page = {
    let kernel_addr_first =
      load_segment
        .iter()
        .fold(u64::MAX, |a, e| if a < e.begin { a } else { e.begin });
    let kernel_addr_last = load_segment
      .iter()
      .fold(0, |a, e| if a > e.end { a } else { e.begin });
    let page_count: usize = ((kernel_addr_last - kernel_addr_first + 0xfff) / 0x1000) as usize;

    writeln!(
      table.stdout(),
      "[Debug] begin: {}, last: {}, pages: {}",
      kernel_addr_first,
      kernel_addr_last,
      page_count,
    )
    .unwrap();

    /*
    table
    .boot_services()
      .allocate_pages(
        AllocateType::Address(kernel_addr_first as usize),
        MemoryType::LOADER_DATA,
        page_count,
      )
      .unwrap_success()
      */
    0
  };
  println!("[Debug] allocated: {}", kernel_page);
  // load kernel to memory
  println!("====================================================");
  let (kernel_ptr, _, _) = loader_pool.into_raw_parts();
  println!("[Debug] ptr: {}", kernel_ptr as usize);
  for seg in load_segment.iter() {
    let src = (kernel_ptr as u64 + seg.offset) as *mut u8;
    let dst = seg.vaddr as *mut u8;
    println!(
      "[Debug] copying {:6} byte from {:8} to {:8}",
      seg.file_size as usize, src as u64, dst as u64
    );

    unsafe {
      core::ptr::copy(src, dst, seg.file_size as usize);
      {
        let dst = (dst as u64 + seg.file_size) as *mut u8;
        core::ptr::write_bytes(dst, 0, (seg.memory_size - seg.file_size) as usize);
      }
      for i in 0..32 {
        print!("{:02x}", *((src as u64 + i as u64) as *const u8))
      }
      println!();
      for i in 0..32 {
        print!("{:02x}", *((dst as u64 + i as u64) as *const u8))
      }
      println!();
    }
  }
  println!("====================================================");
  println!("[Debug] finished loading");

  // get memmap
  let memmap_size = table.boot_services().memory_map_size().map_size;
  let mut memmap = MemoryMap {
    list: [MemoryDescriptor::default(); MEMORYMAP_LIST_LEN],
    len:  0,
  };
  let mut buf = [0 as u8; 1024 * 4];
  println!("[Info] Exiting boot services");
  let (memmap_key, memmap_iter) = table.exit_boot_services(handle, &mut buf).unwrap_success();
  serial_println!("[Debug] end of boot services memmap: {}", memmap_iter.len());

  for desc in memmap_iter {
    if is_available_memory(desc.ty) {
      memmap.list[memmap.len] = *desc;
      memmap.len += 1;
    }
  }

  let entry: extern "sysv64" fn(&FrameBuffer, &MemoryMap) =
    unsafe { core::mem::transmute(kernel_entry) };
  serial_println!(
    "[Info] Let's go! (entrypoint: 0x{:x} | 0x{:x})",
    kernel_entry,
    entry as u64
  );
  for i in 0..16 {
    unsafe {
      serial_print!("{:02x}", *((kernel_entry + i as u64) as *const u8));
    }
  }
  serial_println!();
  entry(&frame_buffer, &memmap);

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
