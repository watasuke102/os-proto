use x86_64::{
  registers::control::{Cr3, Cr3Flags},
  structures::paging::*,
  PhysAddr,
};

const EMPTY_PAGE_TABLE: PageTable = PageTable::new();
const PAGE_DIR_COUNT: usize = 64;

static mut PML4_TABLE: PageTable = PageTable::new();
static mut PDP_TABLE: PageTable = PageTable::new();
static mut PAGE_DIRECTORY: [PageTable; PAGE_DIR_COUNT] = [EMPTY_PAGE_TABLE; PAGE_DIR_COUNT];

fn phys_frame_from_table(table: &PageTable) -> PhysFrame {
  PhysFrame::from_start_address(PhysAddr::new((table as *const PageTable) as u64)).unwrap()
}
pub fn init() {
  let page_table_flag = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

  unsafe {
    PML4_TABLE[0].set_frame(phys_frame_from_table(&PDP_TABLE), page_table_flag);
    // PageTable * 64
    for (i, page_directory) in PAGE_DIRECTORY.iter_mut().enumerate() {
      PDP_TABLE[i].set_frame(phys_frame_from_table(&page_directory), page_table_flag);
      // Page Entry * 512 (2MiB * 512 == 1024MiB == 1GiB)
      for (j, page_table) in PAGE_DIRECTORY[i].iter_mut().enumerate() {
        let addr = (i as u64 * Size1GiB::SIZE) + (j as u64 * Size2MiB::SIZE);
        page_table.set_addr(
          PhysAddr::new(addr),
          page_table_flag | PageTableFlags::HUGE_PAGE,
        );
      }
    }
    Cr3::write(phys_frame_from_table(&PML4_TABLE), Cr3Flags::empty());
  }
}
