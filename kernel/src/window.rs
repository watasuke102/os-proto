use crate::{FrameBufferConfig, PixelColor};
use kernel::Vec2;

enum Direction {
  Top,
  Bottom,
  Right,
  Left,
}

#[derive(Clone, Copy)]
struct Window {
  is_valid: bool,
  size:     Vec2<u32>,
}

pub struct WindowManager<'a> {
  drawer:       &'a FrameBufferConfig,
  list:         [Window; 16],
  active_index: u16,
  top_index:    u16,
}

impl WindowManager<'_> {
  pub fn new(drawer: &FrameBufferConfig) -> WindowManager {
    WindowManager {
      drawer:       drawer,
      list:         [Window {
        is_valid: false,
        size:     Vec2::<u32> { x: 0, y: 0 },
      }; 16],
      active_index: 0,
      top_index:    0,
    }
  }
  pub fn draw(&self) {
    for i in 0..16 {
      let index = ((self.top_index + i) % 16) as usize;
      if self.list[index].is_valid {
        self.drawer.write_rect(
          Vec2::<u32> { x: 0, y: 0 },
          self.list[index].size,
          &PixelColor::from_hex(0xeb4034),
          true,
        );
      }
    }
  }
  pub fn add(&mut self, size: Vec2<u32>) {
    let index = ((self.top_index + 1) % 16) as usize;
    self.list[index] = Window {
      is_valid: true,
      size:     size,
    };
    self.top_index += 1;
    self.active_index = index as u16;
  }
}
