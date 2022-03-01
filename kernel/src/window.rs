use crate::{FrameBufferConfig, PixelColor};
use heapless::{
  pool::singleton::arc::{ArcInner, Pool},
  spsc::Queue,
  Arc,
};
use kernel::{Direction, Vec2};

trait Frame {
  fn draw();
}

pub struct Window {
  //parent_container: Arc<FrameContainer>,
  pub color: PixelColor,
}

pub struct FrameContainer {
  //list:             Queue<Arc<dyn Frame>, 16>,
  pub direction:        Direction,
  pub parent_container: Arc<FrameContainer>,
  pub size:             Vec2<u32>,
}

impl Pool for FrameContainer {
  type Data = Window;
  fn ptr() -> &'static heapless::pool::Pool<ArcInner<<window::Window>::Data>> {
    static POOL: heapless::pool::Pool<ArcInner<<window::Window>::Data>> =
      heapless::pool::Pool::new();
    &POOL
  }
}

pub struct FrameManager {
  pub active_container: Arc<FrameContainer>,
}
