#![no_std]
#![no_main]

#[allow(unused_imports)]
use apps;

fn fib(n: u64) -> u64 {
  match n {
    0 => 0,
    1 => 1,
    _ => fib(n - 2) + fib(n - 1),
  }
}

#[no_mangle]
fn _start() -> u64 {
  fib(16)
}
