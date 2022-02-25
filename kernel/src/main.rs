#![no_std]
#![no_main]
#![feature(asm)]

struct FrameBufferConfig {
    frame_buffer: *const u8,
    pixels_per_scan_line: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: i32,
}

struct MemoryMap {
    buffer_size: u64,
    buffer: *const u8,
    map_size: u64,
    map_key: u64,
    descriptor_size: u64,
    descriptor_version: u32,
}

fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

#[no_mangle]
pub extern "sysv64" fn kernel_main(
    config: &mut FrameBufferConfig,
    memmap: &mut MemoryMap,
    _a: *const u8,
) {
    loop {
        hlt();
    }
}
