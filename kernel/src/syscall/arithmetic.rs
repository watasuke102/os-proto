use common::serial_println;

pub extern "sysv64" fn add(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64) {
  serial_println!(
    "[syscall] add: {} + {} + {} + {} + {} + {} = {}",
    a,
    b,
    c,
    d,
    e,
    f,
    a + b + c + d + e + f
  );
}

pub extern "sysv64" fn diff(a: u64, b: u64) {
  serial_println!("[syscall] diff: {} - {} = {}", a, b, a - b);
}
