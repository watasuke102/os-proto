use common::serial_println;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init() {
  unsafe {
    IDT.breakpoint.set_handler_fn(handle_breakpoint);
    IDT.load();
  }
}

extern "x86-interrupt" fn handle_breakpoint(stack_frame: InterruptStackFrame) {
  serial_println!("[Exception] (Breakpoint)\r\n{:?}", stack_frame);
}
