#![no_std]
#![no_main]
#![feature(abi_efiapi)]

mod uefi;
use core::arch::asm;
use core::panic::PanicInfo;
//use uefi::{EfiHandle, EfiSystemTable};

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {}
}
type EfiHandle = u64;
struct EfiSystemTable {
    _b:  [u8; 60],
    out: *mut OutputProtocol,
}
struct OutputProtocol {
    _b1:           u64,
    output_string: unsafe extern "efiapi" fn(*mut OutputProtocol, *mut [u16; 6]),
    _b2:           [u64; 4],
    clear_screen:  unsafe extern "efiapi" fn(*mut OutputProtocol),
}

#[no_mangle]
fn efi_main(_h: EfiHandle, table: *mut EfiSystemTable) {
    unsafe {
        //((*(*table).stdout).clear_screen)((*table).stdout);
        ((*(*table).out).clear_screen)((*table).out);
        loop {
            asm!("hlt");
            asm!("hlt");
            asm!("hlt");
        }
        //(*table).stdout.Print(65);
    }
}
