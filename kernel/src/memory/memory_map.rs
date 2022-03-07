#[derive(Debug)]
#[repr(C)]
pub struct MemoryMap {
  pub buffer_size:        u64,
  pub buffer:             *const u8,
  pub map_size:           u64,
  pub map_key:            u64,
  pub descriptor_size:    u64,
  pub descriptor_version: u32,
}

struct MemoryDescriptor {
  memory_type:     u32,
  physical_start:  usize,
  virtual_start:   usize,
  number_of_pages: u64,
  attribute:       u64,
}

enum MemoryType {
  ReservedMemoryType,
  LoaderCode,
  LoaderData,
  BootServicesCode,
  BootServicesData,
  RuntimeServicesCode,
  RuntimeServicesData,
  ConventionalMemory,
  UnusableMemory,
  ACPIReclaimMemory,
  ACPIMemoryNVS,
  MemoryMappedIO,
  MemoryMappedIOPortSpace,
  PalCode,
  PersistentMemory,
  MaxMemoryType,
}
