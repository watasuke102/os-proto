#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[derive(Debug, Clone, Copy)]
pub enum Direction {
  Vertical,
  Horizontal,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
  pub x: T,
  pub y: T,
}
