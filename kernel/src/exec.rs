use common::{log_debug, serial_println};
use elf_rs::{Elf, ElfFile, ProgramType};

use crate::memory::segment;

pub fn execute_elf(data: &[u8], mut _entry_addr: u64) {
  let Ok(elf) = Elf::from_bytes(data) else {
    serial_println!("Error: failed to parse the file as ELF");
    return;
  };

  for segment in elf.program_header_iter() {
    if segment.ph_type() == ProgramType::LOAD {
      let src = (data.as_ptr() as u64 + segment.offset()) as *mut u8;
      let dst = segment.vaddr() as *mut u8;
      unsafe {
        core::ptr::copy(src, dst, segment.filesz() as usize);
      }
    }
  }

  let ret: u64;
  let (cs, ss) = segment::get_user_segment();
  log_debug!(
    "ss: {}, cs: {}, entry_point: {} (0x{:x})",
    ss,
    cs,
    elf.entry_point(),
    elf.entry_point()
  );
  let user_stack = 0xffff_8000_0010_0000u64;
  unsafe {
    core::arch::asm!(
      "push rbp",
      "mov  rbp, rsp",
      "push {ss}",
      "push {rsp}",
      "push {cs}",
      "push {rip}",
      "retfq",
      ss  = in(reg) ss as u64,
      rsp = in(reg) user_stack,
      cs  = in(reg) cs as u64,
      rip = in(reg) elf.entry_point(),
      out("rax") ret,
      options(nostack)
    );
  }
  serial_println!("Exit: {}", ret);
}
