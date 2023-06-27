#![no_std]
#![no_main]

#[allow(unused_imports)]
use apps;

#[no_mangle]
fn _start() {
  unsafe {
    core::arch::asm!("mov rax, 0", "mov rdi, 0x17", "syscall");
  }
}
