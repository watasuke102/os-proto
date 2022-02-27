#![no_std]
#![no_main]
mod pixel_writer;
use core::panic::PanicInfo;
use kernel::Vec2;
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
            config.write_pixel(Vec2::<u32> { x: x, y: y }, &PixelColor::from_hex(0x32a852));
        }
    }
    let c = PixelColor::from_hex(0x742e94);
    config.write_rect(
        Vec2::<u32> { x: 50, y: 50 },
        Vec2::<u32> { x: 100, y: 100 },
        &c,
        true,
    );
    config.write_rect(
        Vec2::<u32> { x: 250, y: 250 },
        Vec2::<u32> { x: 10, y: 10 },
        &c,
        false,
    );
    /*
     */
    loop {}
}
