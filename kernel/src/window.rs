use alloc::{rc::Rc, vec, vec::Vec};
use common::{frame_buffer::*, rect::Rect, vec2::Vec2};
use core::cell::{Cell, Ref, RefCell};
use kernel::{print, serial_println, Direction};
use x86_64::structures::paging::frame;

type FrameSize = Vec2<u32>;
type FramePos = Vec2<u32>;

trait Frame {
  fn draw(&self, buffer: &FrameBuffer, pos: FramePos, size: FrameSize);
}

pub struct Window {
  pub color: PixelColor,
}
impl Frame for Window {
  fn draw(&self, buffer: &FrameBuffer, pos: FramePos, size: FrameSize) {
    let rect = Rect { begin: pos, size }.shrink(4);
    serial_println!("draw at: {:?} | size: {:?}", pos, size);
    buffer.write_rect_with_border(
      rect.begin,
      rect.size,
      &self.color,
      &PixelColor::from_hex(0x2b2b2b),
      2,
    );
  }
}

pub struct FrameContainer {
  list:          Vec<Rc<RefCell<dyn Frame>>>,
  direction:     Direction,
  active_window: usize,
}
impl FrameContainer {
  fn window_diff(&self, size: FrameSize) -> FrameSize {
    use Direction::*;
    FrameSize {
      x: match self.direction {
        Top | Bottom => 0,
        _ => size.x / self.list.len() as u32,
      },
      y: match self.direction {
        Left | Right => 0,
        _ => size.y / self.list.len() as u32,
      },
    }
  }
  fn children_size(&self, size: FrameSize) -> FrameSize {
    use Direction::*;
    FrameSize {
      x: match self.direction {
        Top | Bottom => size.x,
        _ => size.x / self.list.len() as u32,
      },
      y: match self.direction {
        Left | Right => size.y,
        _ => size.y / self.list.len() as u32,
      },
    }
  }

  fn push_window(&mut self, color: PixelColor) {
    let win = Rc::new(RefCell::new(Window { color }));
    self.list.push(win.clone());
    self.active_window = self.list.len() - 1;
  }
}
impl Frame for FrameContainer {
  fn draw(&self, buffer: &FrameBuffer, pos: FramePos, size: FrameSize) {
    let child_size = self.children_size(size);
    for (i, frame) in self.list.iter().enumerate() {
      frame.borrow().draw(
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
  pub fn new(frame_buffer: &FrameBuffer) -> FrameManager {
    let FrameSize { x, y } = frame_buffer.resolution;
    FrameManager {
      active_container: None,
      head:             None,
      buffer:           Vec::with_capacity((x * y) as usize),
    }
  }
  pub fn add(&mut self, dir: Direction, color: PixelColor) {
    serial_println!("[ADD] dir: {}", dir as u32);
    let new_active_container: Rc<RefCell<FrameContainer>>;

    if let Some(container) = &self.active_container {
      let mut container = container.borrow_mut();
      if container.direction == dir {
        serial_println!("added (same_direction)");
        container.push_window(color);
        return;
      } else {
        let active_index = container.active_window;
        serial_println!("added (different_direction)");
        new_active_container = Rc::new(RefCell::new(FrameContainer {
          list:          vec![
            container.list[active_index].clone(),
            Rc::new(RefCell::new(Window { color })),
          ],
          direction:     dir,
          active_window: 1,
        }));
        container.list[active_index] = new_active_container.clone();
      }
    } else {
      serial_println!("added (container was empty)");
      let win = Rc::new(RefCell::new(Window { color }));
      new_active_container = Rc::new(RefCell::new(FrameContainer {
        list:          vec![win.clone()],
        direction:     dir,
        active_window: 0,
      }));
      self.head = Some(new_active_container.clone());
    }
    self.active_container = Some(new_active_container.clone());
  }

  pub fn draw(&self, frame_buffer: &FrameBuffer) {
    frame_buffer.write_rect(
      Vec2::<u32> { x: 0, y: 0 },
      frame_buffer.resolution,
      &PixelColor::from_hex(0x32a852),
      true,
    );
    if let Some(head) = &self.head {
      head.borrow().draw(
        frame_buffer,
        FramePos { x: 0, y: 0 },
        frame_buffer.resolution,
      );
    }
  }
}
