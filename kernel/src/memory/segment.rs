use x86_64::{
  registers::segmentation::*,
  structures::gdt::{Descriptor, GlobalDescriptorTable},
};

static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

pub fn init() {
  unsafe {
    // Init GDT
    let code_selector = GDT.add_entry(Descriptor::kernel_code_segment());
    let data_selector = GDT.add_entry(Descriptor::kernel_data_segment());
    GDT.load();
    // Init segment registers
    DS::set_reg(SegmentSelector(0));
    ES::set_reg(SegmentSelector(0));
    FS::set_reg(SegmentSelector(0));
    GS::set_reg(SegmentSelector(0));
    CS::set_reg(code_selector);
    SS::set_reg(data_selector);
  }
}
