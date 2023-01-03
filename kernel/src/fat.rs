use core::str::FromStr;

use alloc::{borrow::ToOwned, format, string::String, vec::Vec};
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

fn long_name_from_entry(data: &[u8]) -> String {
  let mut str_data: Vec<u16> = Vec::new();
  let mut c = 0u16;
  let mut f = false;

  for (i, e) in data.iter().enumerate() {
    match i {
      0 | 11 | 12 | 13 | 26 | 27 => continue,
      _ => (),
    }

    c |= (*e as u16) << (8 * f as u8);
    if f {
      if c == 0 {
        break;
      }
      str_data.push(c);
      c = 0;
    }
    f = !f;
  }

  String::from_utf16_lossy(&str_data)
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

    let mut file_name = String::new();

    for i in 0.. {
      let begin_addr = (rootdir_offset + i * 32) as usize;
      let data: &[u8] = &fat.data[begin_addr..begin_addr + 32];
      match data[0] {
        0x00 => break,
        // TODO: handle 0x05
        0xe5 | 0x05 => continue,
        _ => (),
      }

      let file_attrib = data[11];
      match file_attrib {
        0x08 | 0x10 => continue,
        0x0f => {
          file_name = format!("{}{}", long_name_from_entry(data), file_name);
          continue;
        }
        _ => (),
      }

      if file_name.len() == 0 {
        for j in 0..11 {
          let c = data[j];
          if c != 0x20 {
            file_name.push(c as char);
          }
          if j == 7 {
            file_name.push('.');
          }
        }
        if file_name.ends_with(".") {
          file_name.pop();
        }
      }

      fat.files.push(File {
        name:       String::clone(&file_name),
        attrib:     file_attrib,
        size:       u32::from_ne_bytes(data[28..32].try_into().unwrap()) as usize,
        begin_addr: fat
          .addr_from_cluster(u32::from_le_bytes([data[26], data[27], data[20], data[21]])),
      });
      file_name.clear();
    }

    fat
  }

  pub fn data(&self, index: usize) -> &[u8] {
    let file = &self.files[index];
    &self.data[file.begin_addr..file.begin_addr + file.size]
  }
}
