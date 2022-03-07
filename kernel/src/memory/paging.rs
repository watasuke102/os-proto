use x86_64::{
  registers::control::{Cr3, Cr3Flags},
  structures::paging::*,
  PhysAddr,
};

const EMPTY_PAGE_TABLE: PageTable = PageTable::new();
const PAGE_DIR_COUNT: usize = 64;

static mut L4_TABLE: PageTable = EMPTY_PAGE_TABLE;
static mut DIR_POINTER_TABLE: PageTable = EMPTY_PAGE_TABLE;
static mut PAGE_DIR: [PageTable; PAGE_DIR_COUNT] = [EMPTY_PAGE_TABLE; PAGE_DIR_COUNT];

fn phys_frame_from_table(table: &PageTable) -> PhysFrame {
  PhysFrame::from_start_address(PhysAddr::new((table as *const PageTable) as u64)).unwrap()
}
pub fn init() {
  let page_table_flag = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

  unsafe {
    L4_TABLE[0].set_frame(phys_frame_from_table(&DIR_POINTER_TABLE), page_table_flag);
    for (i, page_dir) in PAGE_DIR.iter_mut().enumerate() {
      DIR_POINTER_TABLE[i].set_frame(phys_frame_from_table(&page_dir), page_table_flag);
      for (j, dir_entry) in PAGE_DIR[i].iter_mut().enumerate() {
        let addr = (i as u64 * Size1GiB::SIZE) + (j as u64 * Size2MiB::SIZE);
        dir_entry.set_addr(
          PhysAddr::new(addr),
          page_table_flag | PageTableFlags::HUGE_PAGE,
        );
      }
    }
    Cr3::write(phys_frame_from_table(&L4_TABLE), Cr3Flags::empty());
  }
}
