#![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
  // Vertical
  Top,
  Bottom,
  // Horizontal
  Left,
  Right,
}
