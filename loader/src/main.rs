#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::{fmt::Write, mem};
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::boot::MemoryDescriptor;
use uefi::ResultExt;

#[entry]
fn main(_handle: Handle, mut table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut table).unwrap_success();
    // get memmap
    let memmup_size = table.boot_services().memory_map_size().map_size;
    writeln!(table.stdout(), "size: {}", memmup_size);
    let mut buf = Vec![0; memmup_size];
    let memmap = table.boot_services().memory_map(&mut buf);
    // get GOP
    let gop = unsafe {
        &mut *(table
            .boot_services()
            .locate_protocol::<GraphicsOutput>()
            .unwrap_success()
            .get())
    };
    loop {}
}
