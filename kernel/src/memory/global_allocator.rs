use crate::memory::memory_map::MemoryDescriptor;
use crate::memory::{linked_list_allocator::LinkedListAllocator, memory_map::MemoryMap};
use crate::print;
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use kernel::serial_println;
use spin::{Mutex, MutexGuard};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new(LinkedListAllocator::new());

pub fn init(memmap: &MemoryMap) {
  use core::mem::size_of;
  serial_println!(
    "sizeof: {}, 0: {:X}, 1: {:X}",
    size_of::<MemoryDescriptor>(),
    memmap.descriptor_list,
    memmap.descriptor_list + memmap.descriptor_size as usize
  );
  serial_println!(
    "{}/{} = {}",
    memmap.map_size,
    memmap.descriptor_size,
    memmap.map_size / memmap.descriptor_size
  );
  for i in 0..(memmap.map_size / memmap.descriptor_size) {
    let desc =
      (memmap.descriptor_list + (memmap.descriptor_size * i) as usize) as *const MemoryDescriptor;
    unsafe {
      serial_println!(
        "{:02} {:X}| {}, {}, {}, {}, {}",
        i,
        memmap.descriptor_list + (memmap.descriptor_size * i) as usize,
        (*desc).memory_type,
        (*desc).physical_start,
        (*desc).virtual_start,
        (*desc).number_of_pages,
        (*desc).attribute,
      );
    }
  }
  todo!();
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
