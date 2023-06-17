#![no_std]
#![no_main]
#![feature(vec_into_raw_parts)]

extern crate alloc;
use alloc::{vec, vec::Vec};
use common::{
  log_debug, log_info,
  memory_map::{is_available_memory, MemoryMap, MEMORYMAP_LIST_LEN},
};
use core::arch::asm;
use elf_rs::{Elf, ElfFile, ProgramType};
use uefi::{
  prelude::*,
  proto::{
    loaded_image::LoadedImage,
    media::{
      file::{File, FileAttribute, FileMode, FileType::*, RegularFile},
      fs::SimpleFileSystem,
    },
  },
  table::boot::MemoryDescriptor,
  CStr16,
};

#[derive(Debug)]
struct LoadSegment {
  offset:      u64,
  vaddr:       u64,
  memory_size: u64,
  file_size:   u64,
}

#[entry]
fn main(handle: Handle, mut table: SystemTable<Boot>) -> Status {
  table.stdout().clear().unwrap();
  uefi_services::init(&mut table).unwrap();
  log_info!("Started boot loader");

  // open kernel file
  log_info!("Loading kernel");
  let (mut kernel_file, kernel_size) = open_file(table.boot_services(), cstr16!("kernel.elf"));
  let mut loader_pool = vec![0; kernel_size as usize];
  kernel_file.read(&mut loader_pool).unwrap();
  // calculate LOAD segment range
  let elf = Elf::from_bytes(&loader_pool).unwrap();
  let kernel_entry = elf.entry_point();
  let mut load_segment = Vec::<LoadSegment>::new();
  for program_header in elf.program_header_iter() {
    if program_header.ph_type() == ProgramType::LOAD {
      load_segment.push(LoadSegment {
        offset:      program_header.offset(),
        vaddr:       program_header.vaddr(),
        file_size:   program_header.filesz(),
        memory_size: program_header.memsz(),
      });
    }
  }
  // load kernel to memory
  let (kernel_ptr, _, _) = loader_pool.into_raw_parts();
  for seg in load_segment.iter() {
    let src = (kernel_ptr as u64 + seg.offset) as *mut u8;
    let dst = seg.vaddr as *mut u8;

    unsafe {
      core::ptr::copy(src, dst, seg.file_size as usize);
      let dst = (dst as u64 + seg.file_size) as *mut u8;
      core::ptr::write_bytes(dst, 0, (seg.memory_size - seg.file_size) as usize);
    }
  }

  // open initfs.img
  log_info!("Loading initial fs");
  let (mut initfs, initfs_size) = open_file(table.boot_services(), cstr16!("initfs.img"));
  let mut initfs_pool = vec![0; initfs_size as usize];
  initfs.read(&mut initfs_pool).unwrap();

  // get memmap
  let mut memmap = MemoryMap {
    list: [MemoryDescriptor::default(); MEMORYMAP_LIST_LEN],
    len:  0,
  };
  log_info!("Exiting boot services");
  let (_, memmap_table) = table.exit_boot_services();
  let memmap_iter = memmap_table.entries().into_iter();
  log_debug!("end of boot services (memmap: {})", memmap_iter.len());

  for desc in memmap_iter {
    if is_available_memory(desc.ty) {
      memmap.list[memmap.len] = *desc;
      memmap.len += 1;
    }
  }

  let entry: extern "sysv64" fn(&MemoryMap, &Vec<u8>) =
    unsafe { core::mem::transmute(kernel_entry) };
  log_debug!(
    "Let's go! (entrypoint: 0x{:x} | 0x{:x})",
    kernel_entry,
    entry as u64
  );
  entry(&memmap, &initfs_pool);

  loop {
    unsafe {
      asm!("hlt");
    }
  }
}

/// Open file and return (file: RegularFile, size: u64)
/// Cause panic when try to open directory (specify directory name)
fn open_file(boot_services: &BootServices, name: &CStr16) -> (RegularFile, u64) {
  // open root dir
  let loaded_image = boot_services
    .open_protocol_exclusive::<LoadedImage>(boot_services.image_handle())
    .unwrap();
  let mut dir = boot_services
    .open_protocol_exclusive::<SimpleFileSystem>(loaded_image.device())
    .unwrap()
    .open_volume()
    .unwrap();

  // find file and check file size
  const BUFFER_SIZE: usize = 128;
  let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
  let size = loop {
    match dir.read_entry(&mut buffer).unwrap() {
      Some(file) => {
        if file.file_name() == name {
          break file.file_size();
        }
      }
      None => panic!("`{}` is not found", name),
    }
  };
  // load kernel to pool
  // FileAttribute is invalid in Read-Only open like this
  match dir
    .open(name, FileMode::Read, FileAttribute::READ_ONLY)
    .unwrap()
    .into_type()
    .unwrap()
  {
    Regular(file) => (file, size),
    _ => panic!("Invalid file type"),
  }
}
