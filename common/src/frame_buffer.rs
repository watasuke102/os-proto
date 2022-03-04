use crate::Vec2;
use uefi::proto::console::gop::PixelFormat;

pub struct PixelColor {
  r: u8,
  g: u8,
  b: u8,
}
impl PixelColor {
  pub fn from_rgb(r: u8, g: u8, b: u8) -> PixelColor {
    PixelColor { r, g, b }
  }
  pub fn from_hex(c: u32) -> PixelColor {
    PixelColor {
      r: ((c >> 16) & 0xff) as u8,
      g: ((c >> 8) & 0xff) as u8,
      b: ((c) & 0xff) as u8,
    }
  }
}

impl FrameBuffer {
  pub fn size(&self) -> &Vec2<u32> {
    &self.resolution
  }
  pub fn write_rect(&self, begin: Vec2<u32>, size: Vec2<u32>, c: &PixelColor, fill: bool) {
    // top, bottom
    for x in begin.x..(begin.x + size.x) {
      self.write_pixel(Vec2::<u32> { x, y: begin.y }, c);
      self.write_pixel(
        Vec2::<u32> {
          x,
          y: begin.y + size.y - 1,
        },
        c,
      );
    }
    // left, right
    for y in (begin.y + 1)..(begin.y + size.y - 1) {
      self.write_pixel(Vec2::<u32> { x: begin.x, y }, c);
      self.write_pixel(
        Vec2::<u32> {
          x: begin.x + size.x - 1,
          y,
        },
        c,
      );
    }
    // body
    if fill {
      for y in (begin.y + 1)..(begin.y + size.y - 1) {
        for x in (begin.x + 1)..(begin.x + size.x - 1) {
          self.write_pixel(Vec2::<u32> { x, y }, c);
        }
      }
    }
  }

  pub fn write_pixel(&self, pos: Vec2<u32>, c: &PixelColor) {
    let p = (self.frame_buffer as u64 +
      4 * (pos.x as u32 + pos.y as u32 * self.resolution.y) as u64) as *mut [u8; 4];
    unsafe {
      match self.pixel_format {
        PixelFormat::Rgb => {
          (*p)[0] = c.r;
          (*p)[1] = c.g;
          (*p)[2] = c.b;
        }
        PixelFormat::Bgr => {
          (*p)[0] = c.b;
          (*p)[1] = c.g;
          (*p)[2] = c.r;
        }
        _ => {
          panic!();
        }
      }
    }
  }
}

pub struct FrameBuffer {
  frame_buffer: *mut u8,
  stride:       usize,
  resolution:   Vec2<u32>,
  pixel_format: PixelFormat,
}
