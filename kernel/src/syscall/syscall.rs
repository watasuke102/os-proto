use common::{log_debug, serial_println};
use core::arch::global_asm;
use x86_64::{
  registers::model_specific::{Efer, EferFlags, LStar},
  VirtAddr,
};

// syscall (abi is same as Linux syscall)
// arg: rdi, rsi, rdx, r10, r8, r9
global_asm!(
  r"
handle_syscall_entry:
  push  rbp
  push  rcx
  push  r11
  
  mov   rcx, r10
  call  [syscall_table + 8*rax]

  pop	 r11
  pop	 rcx
  pop  rbp
  sysretq
"
);

extern "sysv64" {
  fn handle_syscall_entry();
}

#[no_mangle]
static syscall_table: [extern "sysv64" fn(u64, u64, u64, u64, u64, u64); 1] = [add];

extern "sysv64" fn add(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64) {
  serial_println!(
    "[syscall] add: {} + {} + {} + {} + {} + {} = {}",
    a,
    b,
    c,
    d,
    e,
    f,
    a + b + c + d + e + f
  );
}

pub fn init() {
  unsafe {
    log_debug!(
      "syscall entry: 0x{:x}, syscall[0]: 0x{:x}",
      handle_syscall_entry as u64,
      add as u64,
    );
    Efer::write(
      EferFlags::LONG_MODE_ACTIVE | EferFlags::LONG_MODE_ENABLE | EferFlags::SYSTEM_CALL_EXTENSIONS,
    );
    LStar::write(VirtAddr::new(handle_syscall_entry as u64))
  }
}
