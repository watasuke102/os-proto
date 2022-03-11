use crate::vec2::Vec2;

pub struct Rect {
  pub begin: Vec2<u32>,
  pub size:  Vec2<u32>,
}

impl Rect {
  pub fn shrink(&self, ratio: i32) -> Self {
    Rect {
      begin: Vec2::<u32> {
        x: (self.begin.x as i32 + ratio) as u32,
        y: (self.begin.y as i32 + ratio) as u32,
      },
      size:  Vec2::<u32> {
        x: (self.size.x as i32 - ratio * 2) as u32,
        y: (self.size.y as i32 - ratio * 2) as u32,
      },
    }
  }
}
