use alloc::string::String;
use common::serial_println;
use core::arch::asm;
use elf_rs::{Elf, ElfFile, ElfType, ProgramType};

pub fn execute_elf(data: &[u8], mut entry_addr: u64) {
  let Ok(elf) = Elf::from_bytes(data) else {
    serial_println!("Error: failed to parse the file as ELF");
    return;
  };

  // FIXME: handle LOAD segment properly
  for section in elf.section_header_iter() {
    let section_name = section
      .section_name()
      .unwrap_or(&[])
      .iter()
      .map(|&c| c as char)
      .collect::<String>();
    if section_name == ".text" {
      entry_addr += section.offset();
      break;
    }
  }

  let ret: u64;
  unsafe {
    asm!(
      "call {}",
      "mov  {}, rdi",
      in(reg) entry_addr,
      out(reg) ret,
    );
  }
  serial_println!("Exit: {}", ret);
}
