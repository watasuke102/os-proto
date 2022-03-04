use core::mem::transmute;
use x86_64::{registers::control::Cr3, structures::paging::*};

const PAGE_SIZE: usize = 512;
const PAGE_DIRECTORY_COUNT: usize = 64;

#[allow(non_upper_case_globals)]
const PAGE_SIZE_4K: usize = 4096;
#[allow(non_upper_case_globals)]
const PAGE_SIZE_2M: usize = 512 * PAGE_SIZE_4K;
#[allow(non_upper_case_globals)]
const PAGE_SIZE_1G: usize = 512 * PAGE_SIZE_2M;

static mut L4_TABLE: [u64; PAGE_SIZE] = [0; PAGE_SIZE];
static mut DIRECTORY_POINTER_TABLE: [u64; PAGE_SIZE] = [0; PAGE_SIZE];
static mut PAGE_DIRECTORY: [[u64; PAGE_SIZE]; PAGE_DIRECTORY_COUNT] =
  [[0; PAGE_SIZE]; PAGE_DIRECTORY_COUNT];

unsafe fn init() {
  L4_TABLE[0] = &DIRECTORY_POINTER_TABLE[0] | 0x003;
  for i in 0..PAGE_DIRECTORY_COUNT {
    DIRECTORY_POINTER_TABLE[i] = transmute::<&[u64; PAGE_SIZE], u64>(&PAGE_DIRECTORY[0]) | 0x003;
    for j in 0..PAGE_SIZE {
      PAGE_DIRECTORY[i][j] = ((i * PAGE_SIZE_1G) + (j * PAGE_SIZE_2M) | 0x083) as u64;
    }
  }
}
