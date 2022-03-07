use crate::memory::memory_map::MemoryDescriptor;
use crate::memory::{linked_list_allocator::LinkedListAllocator, memory_map::MemoryMap};
use crate::print;
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use kernel::serial_println;
use spin::{Mutex, MutexGuard};

use super::memory_map::MemoryType;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new(LinkedListAllocator::new());

const UEFI_PAGE_SIZE: usize = 4096;

#[rustfmt::skip]
fn is_available_memory(typ: u32) -> bool {
  typ == MemoryType::BootServicesCode   as u32 ||
  typ == MemoryType::BootServicesData   as u32 ||
  typ == MemoryType::ConventionalMemory as u32
}

pub fn init(memmap: &MemoryMap) {
  let mut available_end: usize = 0;
  for i in 0..(memmap.map_size / memmap.descriptor_size) {
    let desc = unsafe {
      let ptr = memmap.descriptor_list + (memmap.descriptor_size * i) as usize;
      &*(ptr as *const MemoryDescriptor)
    };
    let size = desc.number_of_pages as usize * UEFI_PAGE_SIZE;

    if is_available_memory(desc.memory_type) {
      if available_end >= desc.physical_start {
        unsafe {
          ALLOCATOR.lock().add_free_region(desc.physical_start, size);
        }
      }
      available_end = desc.physical_start + size;
    }
  }
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
