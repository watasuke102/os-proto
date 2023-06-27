use core::arch::global_asm;

use common::{log_debug, serial_println};
use elf_rs::{Elf, ElfFile, ProgramType};

use crate::memory::segment;

#[no_mangle]
static mut KERNEL_RSP: u64 = 0;

// call_app(rdi = ss, rsi = cs, rdx = rsp, rcx = rip)
global_asm!(
  r"
call_app:
  lea  rax, [end_call_app]
  push rax
  xor  rax, rax
  mov  [KERNEL_RSP], rsp
  push rbp
  mov  rbp, rsp
  push rdi
  push rdx
  push rsi
  push rcx
  retfq
end_call_app:
  ret
"
);

extern "sysv64" {
  fn call_app(ss: u64, cs: u64, rsp: u64, rip: u64) -> u64;
}

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

  let (cs, ss) = segment::get_user_segment();
  log_debug!(
    "ss: {}, cs: {}, entry_point: {} (0x{:x})",
    ss,
    cs,
    elf.entry_point(),
    elf.entry_point()
  );
  let user_stack = 0xffff_8000_0010_0000u64;
  let ret = unsafe { call_app(ss as u64, cs as u64, user_stack, elf.entry_point()) };
  serial_println!("Exit: {}", ret);
}
