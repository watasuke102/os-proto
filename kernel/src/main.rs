#![no_std]
#![no_main]
mod pixel_writer;
use core::panic::PanicInfo;
use pixel_writer::*;

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
            config.write_pixel((x, y), &PixelColor::from_hex(0x32a852));
        }
    }
    let c = PixelColor::from_hex(0x742e94);
    config.write_rect((50, 50), (100, 100), &c, true);
    config.write_rect((250, 250), (10, 10), &c, false);
    /*
     */
    loop {}
}
