use crate::{FrameBufferConfig, PixelColor};
use heapless::{
  pool::singleton::arc::{ArcInner, Pool},
  spsc::Queue,
  Arc,
};
use kernel::{Direction, Vec2};

pub struct Window {
  valid:     bool,
  pub color: PixelColor,
}

pub struct FrameContainer {
  valid:         bool,
  list:          Queue<Frame, 16>,
  pub direction: Direction,
  pub size:      Vec2<u32>,
}

enum FrameType {
  FrameContainer,
  Window,
}

struct Frame {
  frame_type: FrameType,
  container:  FrameContainer,
  window:     Window,
}

pub struct FrameManager {
  pub active_container: Frame,
}
