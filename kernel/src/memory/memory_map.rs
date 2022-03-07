#[derive(Debug)]
#[repr(C)]
pub struct MemoryMap {
  pub buffer_size:        u64,
  pub descriptor_list:    usize,
  pub map_size:           u64,
  pub map_key:            u64,
  pub descriptor_size:    u64,
  pub descriptor_version: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MemoryDescriptor {
  pub memory_type:     u32,
  pub physical_start:  usize,
  pub virtual_start:   usize,
  pub number_of_pages: u64,
  pub attribute:       u64,
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
