use crate::linked_list::ListNode;
use alloc::alloc::Layout;
use core::mem::{align_of, size_of};
use x86_64::align_up;

pub struct LinkedListAllocator {
  head: ListNode,
}

impl LinkedListAllocator {
  pub const fn new() -> LinkedListAllocator {
    LinkedListAllocator {
      head: ListNode::new(0),
    }
  }

  /// return (size, align)
  pub fn size_align(layout: Layout) -> (usize, usize) {
    let layout = layout
      .align_to(align_of::<ListNode>())
      .expect("failed to adjust alignment")
      .pad_to_align();
    (layout.size().max(size_of::<ListNode>()), layout.align())
  }

  pub unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
    if addr == 0 {
      return;
    }

    assert_eq!(
      align_up(addr as u64, align_of::<ListNode>() as u64) as u64,
      addr as u64
    );
    assert!(size >= size_of::<ListNode>());

    let mut node = ListNode::new(size);
    node.next = self.head.next.take();
    let node_ptr = addr as *mut ListNode;
    node_ptr.write(node);
    self.head.next = Some(&mut *node_ptr)
  }

  pub fn find_region(
    &mut self,
    size: usize,
    align: usize,
  ) -> Option<(&'static mut ListNode, usize)> {
    let mut current = &mut self.head;

    while let Some(ref mut region) = current.next {
      if let Ok(alloc_begin) = LinkedListAllocator::alloc_from_region(&region, size, align) {
        let next = region.next.take();
        let ret = Some((current.next.take().unwrap(), alloc_begin));
        current.next = next;
        return ret;
      } else {
        current = current.next.as_mut().unwrap();
      }
    }

    // region not found
    None
  }

  pub fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
    let alloc_begin = align_up(region.start_addr() as u64, align as u64) as usize;
    let alloc_end = alloc_begin.checked_add(size).ok_or(())? as usize;

    if alloc_end > region.end_addr() {
      return Err(());
    }

    let excess_size = region.end_addr() - alloc_end;
    if excess_size > 0 && excess_size < size_of::<ListNode>() {
      return Err(());
    }

    Ok(alloc_begin)
  }

  pub fn total_size(&mut self) -> usize {
    let mut sum: usize = 0;
    let mut current = &mut self.head;

    while let Some(ref mut region) = current.next {
      sum += region.size;
      current = current.next.as_mut().unwrap();
    }

    sum
  }
}
