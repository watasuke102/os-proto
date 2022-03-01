use kernel::Vec2;

pub struct PixelColor {
  r: u8,
  g: u8,
  b: u8,
}
impl PixelColor {
  pub fn from_rgb(r: u8, g: u8, b: u8) -> PixelColor {
    PixelColor { r: r, g: g, b: b }
  }
  pub fn from_hex(c: u32) -> PixelColor {
    PixelColor {
      r: ((c >> 16) & 0xff) as u8,
      g: ((c >> 8) & 0xff) as u8,
      b: ((c) & 0xff) as u8,
    }
  }
}

enum PixelFormat {
  RgbResv8bit,
  BgrResv8bit,
}

pub struct FrameBufferConfig {
  frame_buffer:          *mut [u8; 4],
  pixels_per_scan_line:  u32,
  horizontal_resolution: u32,
  vertical_resolution:   u32,
  pixel_format:          PixelFormat,
}

impl FrameBufferConfig {
  pub fn size(&self) -> Vec2<u32> {
    Vec2::<u32> {
      x: self.horizontal_resolution,
      y: self.vertical_resolution,
    }
  }
  pub fn write_rect(&self, begin: Vec2<u32>, size: Vec2<u32>, c: &PixelColor, fill: bool) {
    // top, bottom
    for x in begin.x..(begin.x + size.x) {
      self.write_pixel(Vec2::<u32> { x: x, y: begin.y }, c);
      self.write_pixel(
        Vec2::<u32> {
          x: x,
          y: begin.y + size.y - 1,
        },
        c,
      );
    }
    // left, right
    for y in (begin.y + 1)..(begin.y + size.y - 1) {
      self.write_pixel(Vec2::<u32> { x: begin.x, y: y }, c);
      self.write_pixel(
        Vec2::<u32> {
          x: begin.x + size.x - 1,
          y: y,
        },
        c,
      );
    }
    // body
    if fill {
      for y in (begin.y + 1)..(begin.y + size.y - 1) {
        for x in (begin.x + 1)..(begin.x + size.x - 1) {
          self.write_pixel(Vec2::<u32> { x: x, y: y }, c);
        }
      }
    }
  }

  pub fn write_pixel(&self, pos: Vec2<u32>, c: &PixelColor) {
    let p = (self.frame_buffer as u64 +
      4 * (pos.x as u32 + pos.y as u32 * self.horizontal_resolution) as u64)
      as *mut [u8; 4];
    unsafe {
      match self.pixel_format {
        PixelFormat::RgbResv8bit => {
          (*p)[0] = c.r;
          (*p)[1] = c.g;
          (*p)[2] = c.b;
        }
        PixelFormat::BgrResv8bit => {
          (*p)[0] = c.b;
          (*p)[1] = c.g;
          (*p)[2] = c.r;
        }
      }
    }
  }
}
