use common::{log_debug, log_info};
use x86_64::{
  instructions::interrupts,
  structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init() {
  interrupts::disable();
  log_debug!("initing interrupt...");
  unsafe {
    IDT.breakpoint.set_handler_fn(handle_breakpoint);
    IDT.double_fault.set_handler_fn(handle_doublefault);
    IDT.load();
  }
  interrupts::enable();
  log_debug!("Interrupt enabled");
}

extern "x86-interrupt" fn handle_breakpoint(stack_frame: InterruptStackFrame) {
  log_info!("Breakpoint Exception\r\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn handle_doublefault(
  stack_frame: InterruptStackFrame,
  error_code: u64,
) -> ! {
  panic!("DOUBLE FAULT!! ({})\r\n{:#?}", error_code, stack_frame)
}
