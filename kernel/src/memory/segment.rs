use x86_64::{
  registers::segmentation::*,
  structures::gdt::{Descriptor, GlobalDescriptorTable},
};

pub struct MemoryMap {
  buffer_size:        u64,
  buffer:             *const u8,
  map_size:           u64,
  map_key:            u64,
  descriptor_size:    u64,
  descriptor_version: u32,
}

static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

pub unsafe fn init() {
  // Init GDT
  GDT.add_entry(Descriptor::user_code_segment());
  GDT.add_entry(Descriptor::user_data_segment());
  GDT.load();

  // Init segment registers
  DS::set_reg(SegmentSelector(0));
  ES::set_reg(SegmentSelector(0));
  FS::set_reg(SegmentSelector(0));
  GS::set_reg(SegmentSelector(0));

  CS::set_reg(SegmentSelector(1 << 3));
  SS::set_reg(SegmentSelector(2 << 3));
}
