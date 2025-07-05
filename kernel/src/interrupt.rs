use common::{log_debug, log_info};
use x86_64::{
  instructions::interrupts,
  registers::control::Cr2,
  structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

#[allow(static_mut_refs)]
pub fn init() {
  interrupts::disable();
  log_debug!("initing interrupt...");
  unsafe {
    IDT.breakpoint.set_handler_fn(handle_breakpoint);
    // waiting for fix -> https://github.com/rust-osdev/x86_64/issues/553
    // IDT.double_fault.set_handler_fn(handle_doublefault);
    IDT.page_fault.set_handler_fn(handle_pagefault);
    IDT
      .general_protection_fault
      .set_handler_fn(handle_general_protection);
    IDT.load();
  }
  interrupts::enable();
  log_debug!("Interrupt enabled");
}

extern "x86-interrupt" fn handle_breakpoint(stack_frame: InterruptStackFrame) {
  log_info!("Breakpoint Exception\r\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn handle_general_protection(frame: InterruptStackFrame, error: u64) {
  panic!(
    "General Protection Exception (0x{:x} : {})\r\n{:#?}",
    error,
    // TODO: is this correct?
    // https://wiki.osdev.org/General_Protection_Fault#General_Protection_Fault
    match (error & 0b110) >> 1 {
      0b00 => {
        if error == 0 {
          "Unknown error"
        } else {
          "GDT error"
        }
      }
      0b01 => "IDT error",
      0b10 => "LDT error",
      0b11 => "IDT error",
      _ => "Unknown error",
    },
    frame
  );
}
extern "x86-interrupt" fn handle_pagefault(frame: InterruptStackFrame, error: PageFaultErrorCode) {
  panic!(
    "[Exception] (Pagefault)\r\nAccessed: {:?}\r\nframe: {:#?}\r\nerr: {:#?}",
    Cr2::read(),
    frame,
    error
  )
}
// extern "x86-interrupt" fn handle_doublefault(
//   stack_frame: InterruptStackFrame,
//   error_code: u64,
// ) -> ! {
//   panic!("DOUBLE FAULT!! ({})\r\n{:#?}", error_code, stack_frame)
// }
