#![no_std]
#![no_main]

#[allow(unused_imports)]
use apps;

extern "sysv64" fn call_add_syscall(_a: u64, _b: u64, _c: u64, _d: u64, _e: u64, _f: u64) {
  unsafe {
    core::arch::asm!(
      "push rax",
      "mov  rax, 1",
      "syscall",
      "pop rax",
      in("r10") _d,
      options(nostack)
    );
  }
}

#[no_mangle]
fn _start() -> u64 {
  call_add_syscall(1, 2, 3, 4, 5, 6);
  loop {}
}
