#![no_std]
#![no_main]
#![allow(dead_code)]
mod pixel_writer;
mod window;
use core::panic::PanicInfo;
use kernel::Vec2;
use pixel_writer::*;
use window::*;

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
  // config.write_pixel(Vec2::<u32> { x: x, y: y }, &PixelColor::from_hex(0x32a852));
  config.write_rect(
    Vec2::<u32> { x: 0, y: 0 },
    config.size(),
    &PixelColor::from_hex(0x32a852),
    true,
  );

  let mut window_manager = WindowManager::new(config);
  window_manager.add(Vec2::<u32> { x: 200, y: 200 });
  window_manager.draw();

  loop {}
}
