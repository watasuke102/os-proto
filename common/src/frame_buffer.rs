use crate::vec2::Vec2;
use uefi::proto::console::gop::PixelFormat;

#[derive(Clone, Copy)]
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

pub struct FrameBuffer {
  pub frame_buffer: *mut [u8; 4],
  pub stride:       usize,
  pub resolution:   Vec2<u32>,
  pub pixel_format: PixelFormat,
}

impl FrameBuffer {
  pub fn pixel_len(&self) -> usize {
    (self.resolution.x * self.resolution.y) as usize
  }
  pub fn write_pixel(&self, pos: Vec2<u32>, c: &PixelColor) {
    let p = (self.frame_buffer as u64 +
      4 * (pos.x as u32 + pos.y as u32 * self.resolution.x) as u64) as *mut [u8; 4];
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
        _ => panic!(
          "Not implemented pixel format ({})",
          self.pixel_format as u32
        ),
      }
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

  pub fn write_rect_with_border(
    &self,
    begin: Vec2<u32>,
    size: Vec2<u32>,
    body_color: &PixelColor,
    border_color: &PixelColor,
    border_size: u32,
  ) {
    let mut diff = Vec2::<u32> { x: 0, y: 0 };
    let two = Vec2::<u32> { x: 2, y: 2 };
    for _ in 0..border_size {
      self.write_rect(begin + diff, size - diff * two, border_color, false);
      diff.x += 1;
      diff.y += 1;
    }
    self.write_rect(begin + diff, size - diff * two, body_color, true);
  }
}
