use core::str::FromStr;

use alloc::{borrow::ToOwned, string::String, vec::Vec};
use common::{serial_print, serial_println};

#[derive(Debug)]
pub struct File {
  pub name:   String,
  pub size:   usize,
  pub attrib: u8,
  begin_addr: usize,
}

pub struct Fat {
  reserved_section_cnt: u16,
  fats_num:             u8,
  block_per_fat:        u32,
  sector_per_cluster:   u8,
  bytes_per_sector:     u16,
  data:                 Vec<u8>,
  pub files:            Vec<File>,
}

impl Fat {
  fn addr_from_cluster(&self, cluster: u32) -> usize {
    if cluster == 0 {
      return 0usize;
    }

    (self.reserved_section_cnt as usize +
      self.fats_num as usize * self.block_per_fat as usize +
      (cluster - 2) as usize * self.sector_per_cluster as usize) *
      self.bytes_per_sector as usize
  }

  pub fn new(data: Vec<u8>) -> Self {
    let mut fat = Fat {
      reserved_section_cnt: u16::from_le_bytes(data[14..16].try_into().unwrap()),
      fats_num:             data[16],
      block_per_fat:        u32::from_le_bytes(data[36..40].try_into().unwrap()),
      sector_per_cluster:   data[13],
      bytes_per_sector:     u16::from_le_bytes(data[11..13].try_into().unwrap()),
      data:                 data,
      files:                Vec::new(),
    };

    let rootdir_offset =
      fat.addr_from_cluster(u32::from_le_bytes(fat.data[44..48].try_into().unwrap()));

    for i in 0.. {
      let begin_addr = (rootdir_offset + i * 32) as usize;
      match fat.data[begin_addr] {
        0x00 => break,
        // TODO: handle 0x05
        0xe5 | 0x05 => continue,
        _ => (),
      }

      let file_attrib = fat.data[begin_addr + 11];
      match file_attrib {
        0x08 | 0x10 | 0x0f => continue,
        _ => (),
      }

      let mut file = File {
        name:       String::new(),
        attrib:     file_attrib,
        size:       u32::from_ne_bytes(
          fat.data[begin_addr + 28..begin_addr + 28 + 4]
            .try_into()
            .unwrap(),
        ) as usize,
        begin_addr: fat.addr_from_cluster(u32::from_le_bytes([
          fat.data[begin_addr + 26],
          fat.data[begin_addr + 27],
          fat.data[begin_addr + 20],
          fat.data[begin_addr + 21],
        ])),
      };
      // name
      for j in 0..11 {
        let c = fat.data[begin_addr + j];
        if c != 0x20 {
          file.name.push(c as char);
        }
        if j == 7 {
          file.name.push('.');
        }
      }
      if file.name.ends_with(".") {
        file.name.pop();
      }

      fat.files.push(file);
    }

    fat
  }

  pub fn data(&self, index: usize) -> &[u8] {
    let file = &self.files[index];
    &self.data[file.begin_addr..file.begin_addr + file.size]
  }
}
