use x86_64::{
  registers::control::{Cr3, Cr3Flags},
  structures::paging::*,
  PhysAddr,
};

const EMPTY_PAGE_TABLE: PageTable = PageTable::new();
const PAGE_DIR_COUNT: usize = 64;

static mut PML4_TABLE: PageTable = PageTable::new();
static mut KERNEL_PDP_TABLE: PageTable = PageTable::new();
static mut KERNEL_PAGE_DIRECTORY: [PageTable; PAGE_DIR_COUNT] = [EMPTY_PAGE_TABLE; PAGE_DIR_COUNT];
static mut USER_PDP_TABLE: PageTable = PageTable::new();
static mut USER_PAGE_DIRECTORY: PageTable = PageTable::new();

fn phys_frame_from_table(table: &PageTable) -> PhysFrame {
  PhysFrame::from_start_address(PhysAddr::new((table as *const PageTable) as u64)).unwrap()
}
pub fn init() {
  // TODO: remove PRESENT (should be set at PageFault handler?)
  let kernel_page_flag = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
  let user_page_flag = kernel_page_flag | PageTableFlags::USER_ACCESSIBLE;

  // Page Entry * 512 (2MiB * 512 == 1024MiB == 1GiB)
  unsafe {
    // kernel : p4_index == 0x000
    PML4_TABLE[0].set_frame(phys_frame_from_table(&KERNEL_PDP_TABLE), kernel_page_flag);
    // kernel space: 0x0000_0000_0000_0000 - 64GB
    for (i, page_directory) in KERNEL_PAGE_DIRECTORY.iter_mut().enumerate() {
      KERNEL_PDP_TABLE[i].set_frame(phys_frame_from_table(&page_directory), kernel_page_flag);
      for (j, page_table) in KERNEL_PAGE_DIRECTORY[i].iter_mut().enumerate() {
        let addr = (i as u64 * Size1GiB::SIZE) + (j as u64 * Size2MiB::SIZE);
        page_table.set_addr(
          PhysAddr::new(addr),
          kernel_page_flag | PageTableFlags::HUGE_PAGE,
        );
      }
    }

    // user : p4_index == 0x100 (== 256)
    PML4_TABLE[0x100].set_frame(phys_frame_from_table(&USER_PDP_TABLE), user_page_flag);
    USER_PDP_TABLE[0].set_frame(phys_frame_from_table(&USER_PAGE_DIRECTORY), user_page_flag);
    // user space: 0xffff_8000_0000_0000 - 1GB
    for i in 0..512 {
      USER_PAGE_DIRECTORY[i].set_addr(
        PhysAddr::new(i as u64 * Size2MiB::SIZE + Size1GiB::SIZE),
        user_page_flag | PageTableFlags::HUGE_PAGE,
      );
    }
    Cr3::write(phys_frame_from_table(&PML4_TABLE), Cr3Flags::empty());
  }
}
