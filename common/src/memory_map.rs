use alloc::vec::Vec;
use uefi::table::boot::{MemoryDescriptor, MemoryType};

pub struct MemoryMap {
  pub list: Vec<MemoryDescriptor>,
  pub len:  usize,
}

#[rustfmt::skip]
pub fn is_available_memory(typ: MemoryType) -> bool {
  typ == MemoryType::BOOT_SERVICES_CODE ||
  typ == MemoryType::BOOT_SERVICES_DATA ||
  typ == MemoryType::CONVENTIONAL      
}
