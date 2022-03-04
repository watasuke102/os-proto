#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use common::{frame_buffer::*, Vec2};
use core::{arch::asm, fmt::Write, mem};
use uefi::{prelude::*, proto::console::gop::GraphicsOutput, ResultExt};

#[entry]
fn main(_handle: Handle, mut table: SystemTable<Boot>) -> Status {
    table.stdout().clear().unwrap();
    uefi_services::init(&mut table).unwrap_success();
    writeln!(table.stdout(), "[Log] Started boot loader");
    // get memmap
    /*
    let memmup_size = table.boot_services().memory_map_size().map_size;
    writeln!(table.stdout(), "size: {}", memmup_size);
    let mut buf = Vec![0; memmup_size];
    let memmap = table.boot_services().memory_map(&mut buf);
    */
    // get GOP
    writeln!(table.stdout(), "[Log] Loading GOP");
    let gop = unsafe {
        &mut *(table
            .boot_services()
            .locate_protocol::<GraphicsOutput>()
            .unwrap_success()
            .get())
    };
    let frame_buffer = FrameBuffer {
        frame_buffer: gop.frame_buffer().as_mut_ptr(),
        stride:       gop.current_mode_info().stride(),
        resolution:   Vec2::<u32> {
            x: gop.current_mode_info().resolution().0 as u32,
            y: gop.current_mode_info().resolution().1 as u32,
        },
        pixel_format: gop.current_mode_info().pixel_format(),
    };

    // read ELF
    writeln!(table.stdout(), "[Log] Loading root dir");
    let file_system = table
        .boot_services()
        .get_image_file_system(_handle)
        .unwrap_success();
    let dir = unsafe { &mut *file_system.interface.get() }
        .open_volume()
        .unwrap_success();

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
