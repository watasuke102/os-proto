use uefi::table::boot::{MemoryDescriptor, MemoryType};

pub const MEMORYMAP_LIST_LEN: usize = 32;

pub struct MemoryMap {
  pub list: [MemoryDescriptor; MEMORYMAP_LIST_LEN],
  pub len:  usize,
}

#[rustfmt::skip]
pub fn is_available_memory(typ: MemoryType) -> bool {
  typ == MemoryType::BOOT_SERVICES_CODE ||
  typ == MemoryType::BOOT_SERVICES_DATA ||
  typ == MemoryType::CONVENTIONAL      
}
