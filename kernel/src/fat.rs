use core::str::FromStr;

use alloc::{string::String, vec::Vec};
use common::{serial_print, serial_println};

#[derive(Debug)]
pub struct File {
  pub name:   String,
  pub size:   usize,
  pub attrib: u8,
}

pub struct Fat {
  data:      Vec<u8>,
  pub files: Vec<File>,
}

impl Fat {
  pub fn new(data: Vec<u8>) -> Self {
    let mut fat = Fat {
      data:  data,
      files: Vec::new(),
    };

    let rootdir_offset = {
      let rootdir_block = {
        let rsvd_sec_cnt = u16::from_ne_bytes(fat.data[14..16].try_into().unwrap()) as u32;
        let fats_num = fat.data[16] as u32;
        let fats_z32 = u32::from_ne_bytes(fat.data[36..40].try_into().unwrap());
        let root_clus = u32::from_ne_bytes(fat.data[44..48].try_into().unwrap());
        let sec_per_clus = fat.data[13] as u32;
        rsvd_sec_cnt + fats_num * fats_z32 + (root_clus - 2) * sec_per_clus
      };
      serial_println!("[Debug] root_block: {:x}", rootdir_block);
      let bytes_per_sec = u16::from_ne_bytes(fat.data[11..13].try_into().unwrap()) as u32;
      rootdir_block * bytes_per_sec
    };

    for i in 0.. {
      let begin_addr = (rootdir_offset + i * 32) as usize;
      match fat.data[begin_addr] {
        0x00 => break,
        // TODO: handle 0x05
        0xe5 | 0x05 => continue,
        _ => (),
      }
      let mut file = File {
        name:   String::new(),
        attrib: fat.data[begin_addr + 11],
        size:   u32::from_ne_bytes(
          fat.data[begin_addr + 28..begin_addr + 28 + 4]
            .try_into()
            .unwrap(),
        ) as usize,
      };

      match file.attrib {
        0x08 | 0x10 | 0x0f => continue,
        _ => (),
      }

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
}
