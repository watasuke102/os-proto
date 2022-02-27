#![no_std]
#![no_main]
mod pixel_writer;
use core::panic::PanicInfo;
use pixel_writer::*;

pub struct FrameBufferConfig {
    frame_buffer:          *mut PixelColor,
    pixels_per_scan_line:  u32,
    horizontal_resolution: u32,
    vertical_resolution:   u32,
    pixel_format:          i32,
}

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "sysv64" fn kernel_main(
    config: &mut FrameBufferConfig,
    _memmap: u64,
    _acpi_table: u64,
) -> ! {
    for y in 0..config.vertical_resolution {
        for x in 0..config.horizontal_resolution {
            unsafe {
                let p = (config.frame_buffer as u64 +
                    4 * (x + y * config.horizontal_resolution) as u64)
                    as *mut PixelColor;
                (*p) = PixelColor::from_hex(0x32a852);
            }
        }
    }
    loop {}
}
