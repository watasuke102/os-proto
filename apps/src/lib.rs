#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
pub fn handle_panic(_: &PanicInfo) -> ! {
  loop {}
}

pub fn f() -> u64 {
  128u64
}

pub mod syscall {
  #[inline]
  pub extern "sysv64" fn exit(exitcode: u64) -> ! {
    unsafe {
      core::arch::asm!("mov rax, 0", "syscall", in("rdi") exitcode);
    }
    unreachable!()
  }

  #[inline]
  pub extern "sysv64" fn add(_a: u64, _b: u64, _c: u64, _d: u64, _e: u64, _f: u64) {
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

  #[inline]
  pub extern "sysv64" fn diff(_a: u64, _b: u64) {
    unsafe {
      core::arch::asm!(
        "push rax",
        "mov  rax, 2",
        "syscall",
        "pop rax",
        options(nostack)
      );
    }
  }
}
