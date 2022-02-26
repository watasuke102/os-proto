#![no_std]
#![no_main]
use core::mem::align_of;
use core::panic::PanicInfo;

pub struct FrameBufferConfig {
    frame_buffer: *mut [u8; 4],
    pixels_per_scan_line: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: i32,
}

pub struct MemoryMap {
    buffer_size: u64,
    buffer: *const u8,
    map_size: u64,
    map_key: u64,
    descriptor_size: u64,
    descriptor_version: u32,
}

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "sysv64" fn kernel_main(
    config: &mut FrameBufferConfig,
    _memmap: &mut MemoryMap,
    _a: *const u8,
) -> ! {
    for y in 0..config.vertical_resolution {
        for x in 0..config.horizontal_resolution {
            unsafe {
                let p = (config.frame_buffer as u64
                    + 4 * (x + y * config.horizontal_resolution) as u64)
                    as *mut [u8; 4];
                (*p)[0] = 0xff;
                (*p)[1] = 0xff;
                (*p)[2] = 0xff;
            }
        }
    }
    loop {}
}
