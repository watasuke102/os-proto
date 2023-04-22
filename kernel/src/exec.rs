use alloc::string::String;
use common::serial_println;
use core::arch::asm;
use elf_rs::{Elf, ElfFile, ElfType, ProgramType};
use x86_64::{PhysAddr, VirtAddr};

use crate::memory;

pub fn execute_elf(data: &[u8], entry_addr: u64) {
  let Ok(elf) = Elf::from_bytes(data) else {
    serial_println!("Error: failed to parse the file as ELF");
    return;
  };

  // if elf.elf_header().elftype() != ElfType::ET_EXEC {
  //   serial_println!("Error: invalid ELF type ({:?})", elf.elf_header().elftype());
  //   return;
  // }

  copy_load_segment(&elf, data);

  let ret: u64;
  unsafe {
    asm!(
      "call {}",
      "mov  {}, rdi",
      in(reg) elf.entry_point(),
      out(reg) ret,
    );
  }
  serial_println!("Exit: {}", ret);
}

/// return the address of first LOAD semgent
fn copy_load_segment(elf: &Elf, data: &[u8]) {
  for segment in elf.program_header_iter() {
    if segment.ph_type() != ProgramType::LOAD {
      continue;
    }

    unsafe {
      memory::paging::map(
        VirtAddr::new((segment.vaddr() + 4095) / 4096),
        PhysAddr::new((segment.paddr() + 4095) / 4096),
      );
    }

    let src = (((data as *const [u8]) as *const u8) as u64 + segment.offset()) as *const u8;
    unsafe {
      core::ptr::copy(src, segment.vaddr() as *mut u8, segment.filesz() as usize);
      core::ptr::write_bytes(
        (segment.vaddr() + segment.filesz()) as *mut u8,
        0,
        (segment.memsz() - segment.filesz()) as usize,
      );
    }
  }
}
