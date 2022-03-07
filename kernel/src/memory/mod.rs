pub mod linked_list_allocator;
pub mod memory_map;
pub mod paging;
pub mod segment;

mod global_allocator;
pub use global_allocator::init;
