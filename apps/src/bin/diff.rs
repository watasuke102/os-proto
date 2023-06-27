#![no_std]
#![no_main]

#[allow(unused_imports)]
use apps;

extern "sysv64" fn call_diff_syscall(_a: u64, _b: u64) {
  unsafe {
    core::arch::asm!(
      "push rax",
      "mov  rax, 1",
      "syscall",
      "pop rax",
      options(nostack)
    );
  }
}

#[no_mangle]
fn _start() -> u64 {
  call_diff_syscall(8, 4);
  loop {}
}
