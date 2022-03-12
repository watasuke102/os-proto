use crate::memory::linked_list_allocator::LinkedListAllocator;
use crate::serial_println;
use alloc::alloc::{GlobalAlloc, Layout};
use common::memory_map::{is_available_memory, MemoryMap};
use core::ptr::null_mut;
use spin::{Mutex, MutexGuard};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new(LinkedListAllocator::new());

const UEFI_PAGE_SIZE: usize = 4096;

pub fn init(memmap: &MemoryMap) {
  for desc in &memmap.list {
    if is_available_memory(desc.ty) {
      unsafe {
        ALLOCATOR.lock().add_free_region(
          desc.phys_start as usize,
          desc.page_count as usize * UEFI_PAGE_SIZE,
        );
      }
    }
  }

  serial_println!(
    "Allocator inited: total_size={}",
    ALLOCATOR.lock().total_size()
  );
}

struct Allocator {
  item: Mutex<LinkedListAllocator>,
}

impl Allocator {
  pub const fn new(item: LinkedListAllocator) -> Allocator {
    Allocator {
      item: Mutex::new(item),
    }
  }
  pub fn lock(&self) -> MutexGuard<LinkedListAllocator> {
    self.item.lock()
  }
}

unsafe impl GlobalAlloc for Allocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    let (size, align) = LinkedListAllocator::size_align(layout);
    let mut allocator = self.lock();

    if let Some((region, alloc_begin)) = allocator.find_region(size, align) {
      let alloc_end = alloc_begin.checked_add(size).unwrap();
      let excess_size = region.end_addr() - alloc_end;
      if excess_size > 0 {
        allocator.add_free_region(alloc_end, excess_size);
      }
      alloc_begin as *mut u8
    } else {
      null_mut()
    }
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    let (size, _) = LinkedListAllocator::size_align(layout);
    self.lock().add_free_region(ptr as usize, size);
  }
}
