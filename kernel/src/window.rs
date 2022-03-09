use crate::{FrameBufferConfig, PixelColor};
use alloc::{rc::Rc, vec, vec::Vec};
use core::cell::{Cell, RefCell};
use kernel::{print, serial_println, Direction, Vec2};
use x86_64::structures::paging::frame;

type FrameSize = Vec2<u32>;
type FramePos = Vec2<u32>;

trait Frame {
  fn draw(&self, buffer: &FrameBufferConfig, pos: FramePos, size: FrameSize);
}

pub struct Window {
  pub color: PixelColor,
}
impl Frame for Window {
  fn draw(&self, buffer: &FrameBufferConfig, pos: FramePos, size: FrameSize) {
    serial_println!("draw at: {:?} | size: {:?}", pos, size);
    buffer.write_rect_with_border(pos, size, &self.color, &PixelColor::from_hex(0x2b2b2b), 2);
  }
}

pub struct FrameContainer {
  list:          Vec<Rc<dyn Frame>>,
  direction:     Direction,
  active_window: Rc<Window>,
}
impl FrameContainer {
  fn window_diff(&self, size: FrameSize) -> FrameSize {
    use Direction::*;
    FrameSize {
      x: match self.direction {
        Vertical => 0,
        _ => size.x / self.list.len() as u32,
      },
      y: match self.direction {
        Horizontal => 0,
        _ => size.y / self.list.len() as u32,
      },
    }
  }
  fn children_size(&self, size: FrameSize) -> FrameSize {
    use Direction::*;
    FrameSize {
      x: match self.direction {
        Vertical => size.x,
        _ => size.x / self.list.len() as u32,
      },
      y: match self.direction {
        Horizontal => size.y,
        _ => size.y / self.list.len() as u32,
      },
    }
  }

  fn add(&mut self, color: PixelColor) {
    let win = Rc::new(Window { color });
    self.list.push(win.clone());
    self.active_window = win;
  }
}
impl Frame for FrameContainer {
  fn draw(&self, buffer: &FrameBufferConfig, pos: FramePos, size: FrameSize) {
    let child_size = self.children_size(size);
    for (i, frame) in self.list.iter().enumerate() {
      frame.draw(
        buffer,
        pos +
          self.window_diff(size) *
            FramePos {
              x: i as u32,
              y: i as u32,
            },
        child_size,
      );
    }
  }
}

pub struct FrameManager {
  active_container: Option<Rc<RefCell<FrameContainer>>>,
  head:             Option<Rc<RefCell<FrameContainer>>>,
  buffer:           Vec<PixelColor>,
}

impl FrameManager {
  pub fn new(frame_buffer: &FrameBufferConfig) -> FrameManager {
    FrameManager {
      active_container: None,
      head:             None,
      buffer:           Vec::with_capacity(
        (frame_buffer.vertical_resolution * frame_buffer.horizontal_resolution) as usize,
      ),
    }
  }
  pub fn add(&mut self, dir: Direction, col: PixelColor) {
    serial_println!("[ADD] dir: {}", dir as u32);
    if let Some(container) = &self.active_container {
      if container.borrow().direction == dir {
        serial_println!("added(same_direction)");
        container.borrow_mut().add(col);
      }
    } else {
      serial_println!("added (container was empty)");
      let win = Rc::new(Window { color: col });
      self.active_container = Some(Rc::new(RefCell::new(FrameContainer {
        list:          vec![win.clone()],
        direction:     dir,
        active_window: win.clone(),
      })));
      self.head = self.active_container.clone();
    }
  }

  pub fn draw(&self, frame_buffer: &FrameBufferConfig) {
    frame_buffer.write_rect(
      Vec2::<u32> { x: 0, y: 0 },
      frame_buffer.size(),
      &PixelColor::from_hex(0x32a852),
      true,
    );
    if let Some(head) = &self.head {
      head.borrow().draw(
        frame_buffer,
        FramePos { x: 0, y: 0 },
        FrameSize {
          x: frame_buffer.horizontal_resolution,
          y: frame_buffer.vertical_resolution,
        },
      );
    }
  }
}
