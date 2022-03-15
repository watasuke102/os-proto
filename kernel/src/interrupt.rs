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
  let mut keyboard_stat = PortReadOnly::<u8>::new(0x64);
  interrupts::disable();
  serial_println!("[Debug] initing interrupt...");

  unsafe {
    // disable 8259
    PortWriteOnly::<u8>::new(0x21).write(0xff);
    PortWriteOnly::<u8>::new(0xa1).write(0xff);

    *(0xfee0_00f0 as *mut u32) |= 0x100;

    IDT.breakpoint.set_handler_fn(handle_breakpoint);
    IDT.double_fault.set_handler_fn(handle_doublefault);

    for i in 32..128usize {
      IDT[i].set_handler_fn(handle_any);
    }

    IDT[32].set_handler_fn(handle_timer);
    IDT[33].set_handler_fn(handle_keyboard);
    IDT[44].set_handler_fn(handle_mouse);
    IDT.load();
    serial_println!("[Debug] IDT loaded");
    //for i in 0..20_000_000 {}

    // timer
    *(0xfee0_03e0 as *mut u32) = 0b1011;
    *(0xfee0_0320 as *mut u32) = (0b10 << 16) | 32;
    *(0xfee0_0380 as *mut u32) = 0x4fff_ffff;

    // keyboard
    while (keyboard_stat.read() & 0b0010) != 0 {}
    PortWriteOnly::<u8>::new(0x64).write(0x60);
    while (keyboard_stat.read() & 0b0010) != 0 {}
    PortWriteOnly::<u8>::new(0x60).write(0x47);

    // mouse
    while (keyboard_stat.read() & 0b0010) != 0 {}
    PortWriteOnly::<u8>::new(0x64).write(0xd4);
    while (keyboard_stat.read() & 0b0010) != 0 {}
    PortWriteOnly::<u8>::new(0x60).write(0xf4);
  }
  interrupts::enable();
  serial_println!("[Debug] Interrupt enabled");
}

extern "x86-interrupt" fn handle_breakpoint(stack_frame: InterruptStackFrame) {
  serial_println!("[Exception] (Breakpoint)\r\n{:?}", stack_frame);
}
extern "x86-interrupt" fn handle_doublefault(
  stack_frame: InterruptStackFrame,
  error_code: u64,
) -> ! {
  panic!("DOUBLE FAULT!! ({})\r\n{:?}", error_code, stack_frame)
}

fn end_interrupt() {
  unsafe {
    *(0xfee0_00b0 as *mut u32) = 0;
  }
}
extern "x86-interrupt" fn handle_timer(_: InterruptStackFrame) {
  serial_println!("  **Interrupt (timer)");
  end_interrupt();
}
extern "x86-interrupt" fn handle_any(frame: InterruptStackFrame) {
  serial_println!("  **Interrupt (any) {:?}", frame);
  end_interrupt();
}
extern "x86-interrupt" fn handle_mouse(_: InterruptStackFrame) {
  serial_println!("  **Interrupt (mouse)");
  end_interrupt();
}
extern "x86-interrupt" fn handle_keyboard(stack_frame: InterruptStackFrame) {
  unsafe {
    serial_println!(
      "  **Interrupt (keyboard: {}, {}, {})",
      PortReadOnly::<u8>::new(0x60).read(),
      PortReadOnly::<u8>::new(0x60).read(),
      PortReadOnly::<u8>::new(0x60).read(),
    );
    /*
    PortWriteOnly::<u8>::new(0x20).write(0x20);
    PortWriteOnly::<u8>::new(0xa0).write(0x20);
    */
  }
  end_interrupt();
}
