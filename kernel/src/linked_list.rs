pub struct ListNode {
  pub size: usize,
  pub next: Option<&'static mut ListNode>,
}

impl ListNode {
  pub const fn new(size: usize) -> Self {
    ListNode { size, next: None }
  }
  pub fn start_addr(&self) -> usize {
    (self as *const Self) as usize
  }
  pub fn end_addr(&self) -> usize {
    self.start_addr() + self.size
  }
}