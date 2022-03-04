#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod frame_buffer;

#[derive(Debug)]
pub struct Vec2<T> {
  pub x: T,
  pub y: T,
}
