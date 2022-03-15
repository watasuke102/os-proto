use core::fmt;
use core::fmt::{Display, Formatter};
use core::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2<T> {
  pub x: T,
  pub y: T,
}
impl<T: Display> Display for Vec2<T> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl<T: Mul<Output = T>> Mul for Vec2<T> {
  type Output = Self;
  fn mul(self, other: Self) -> Self {
    Self {
      x: self.x * other.x,
      y: self.y * other.y,
    }
  }
}
