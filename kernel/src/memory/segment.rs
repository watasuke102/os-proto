use core::cell::OnceCell;

use x86_64::{
  registers::segmentation::*,
  structures::gdt::{Descriptor, GlobalDescriptorTable},
};

static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();
static mut KERNEL_CODE_SELECTOR: OnceCell<SegmentSelector> = OnceCell::new();
static mut KERNEL_DATA_SELECTOR: OnceCell<SegmentSelector> = OnceCell::new();
static mut USER_CODE_SELECTOR: OnceCell<SegmentSelector> = OnceCell::new();
static mut USER_DATA_SELECTOR: OnceCell<SegmentSelector> = OnceCell::new();

pub fn init() {
  unsafe {
    KERNEL_CODE_SELECTOR
      .set(GDT.add_entry(Descriptor::kernel_code_segment()))
      .unwrap();
    KERNEL_DATA_SELECTOR
      .set(GDT.add_entry(Descriptor::kernel_data_segment()))
      .unwrap();
    USER_DATA_SELECTOR
      .set(GDT.add_entry(Descriptor::user_data_segment()))
      .unwrap();
    USER_CODE_SELECTOR
      .set(GDT.add_entry(Descriptor::user_code_segment()))
      .unwrap();
    GDT.load();
    // Init segment registers
    DS::set_reg(SegmentSelector(0));
    ES::set_reg(SegmentSelector(0));
    FS::set_reg(SegmentSelector(0));
    GS::set_reg(SegmentSelector(0));
    CS::set_reg(*KERNEL_CODE_SELECTOR.get().unwrap());
    SS::set_reg(*KERNEL_DATA_SELECTOR.get().unwrap());
  }
}

// (CS, SS)
pub fn get_user_segment() -> (u16, u16) {
  unsafe {
    (
      (*USER_CODE_SELECTOR.get().unwrap()).0,
      (*USER_DATA_SELECTOR.get().unwrap()).0,
    )
  }
}
