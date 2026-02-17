use crate::syscall::arithmetic::*;
use common::log_debug;
use core::arch::global_asm;
use x86_64::{
  VirtAddr,
  registers::model_specific::{Efer, EferFlags, LStar},
};

// syscall (abi is same as Linux syscall)
// arg: rdi, rsi, rdx, r10, r8, r9
global_asm!(
  r"
handle_syscall_entry:
  cmp   rax, 0
  je    .syscall_exit
  push  rbp
  push  rcx
  push  r11
  
  mov   rcx, r10
  call  [syscall_table + 8*rax]

  pop	 r11
  pop	 rcx
  pop  rbp
  sysretq
.syscall_exit:
  mov rsp, [KERNEL_RSP]
  mov rax, rdi
  ret
"
);

unsafe extern "sysv64" {
  fn handle_syscall_entry();
}

const SYSCALL_LEN: usize = 3;
#[unsafe(no_mangle)]
#[allow(non_upper_case_globals)]
static mut syscall_table: [u64; SYSCALL_LEN] = [0; SYSCALL_LEN];

pub fn init() {
  unsafe {
    syscall_table = [0, add as *const () as u64, diff as *const () as u64];
    log_debug!(
      "syscall entry: 0x{:x}, syscall[0]: 0x{:x}",
      handle_syscall_entry as *const () as u64,
      add as *const () as u64,
    );
    Efer::write(
      EferFlags::LONG_MODE_ACTIVE | EferFlags::LONG_MODE_ENABLE | EferFlags::SYSTEM_CALL_EXTENSIONS,
    );
    LStar::write(VirtAddr::new(handle_syscall_entry as *const () as u64))
  }
}
