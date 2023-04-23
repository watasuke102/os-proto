use common::serial_println;
use x86_64::{
  instructions::{
    interrupts,
    port::{PortReadOnly, PortWriteOnly},
  },
  structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init() {
  interrupts::disable();
  serial_println!("[Debug] initing interrupt...");
  unsafe {
    IDT.breakpoint.set_handler_fn(handle_breakpoint);
    IDT.double_fault.set_handler_fn(handle_doublefault);
    IDT.load();
  }
  interrupts::enable();
  serial_println!("[Debug] Interrupt enabled");
}

extern "x86-interrupt" fn handle_breakpoint(stack_frame: InterruptStackFrame) {
  serial_println!("[Exception] (Breakpoint)\r\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn handle_doublefault(
  stack_frame: InterruptStackFrame,
  error_code: u64,
) -> ! {
  panic!("DOUBLE FAULT!! ({})\r\n{:#?}", error_code, stack_frame)
}
