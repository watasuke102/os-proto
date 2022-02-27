pub struct PixelColor {
  r:         u8,
  g:         u8,
  b:         u8,
  _reserved: u8,
}
impl PixelColor {
  pub fn from_rgb(r: u8, g: u8, b: u8) -> PixelColor {
    PixelColor {
      r:         r,
      g:         g,
      b:         g,
      _reserved: 0,
    }
  }
  pub fn from_hex(c: u32) -> PixelColor {
    PixelColor {
      r:         ((c >> 16) & 0xff) as u8,
      g:         ((c >> 8) & 0xff) as u8,
      b:         ((c) & 0xff) as u8,
      _reserved: 0,
    }
  }
}
