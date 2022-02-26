pub type EfiHandle = u64; // VOID*
struct EfiTableHeader {
  signature:   u64,
  revision:    u32,
  header_size: u32,
  crc32:       u32,
  reserved:    u32,
}
pub struct EfiSystemTable {
  header:            EfiTableHeader,
  firmware_vendor:   u64,
  firmware_revision: u32,
  stdin_handle:      EfiHandle,
  stdin:             protocol::SimpleTextInput,
  stdout_handle:     EfiHandle,
  pub stdout:        *mut protocol::SimpleTextOutput,
  stderr_handle:     EfiHandle,
  stderr:            protocol::SimpleTextOutput,
  runtime_services:  EfiRuntimeServices,
  boot_services:     EfiBootServices,
  table_entry_num:   usize,
  config_table:      EfiConfigulationTable,
}

struct EfiRuntimeServices {
  header: EfiTableHeader,
  a:      [u64; 14],
}
struct EfiBootServices {
  header: EfiTableHeader,
  a:      [u64; 44],
}

// 128-bit
type EfiGuid = [u64; 2];
struct EfiConfigulationTable {
  vendor_guid:  EfiGuid,
  vendor_table: *const EfiGuid,
}

pub mod protocol {
  use core::arch::asm;
  // all fn pointer
  pub struct SimpleTextInput {
    pub reset:           u64,
    pub read_key_stroke: u64,
    pub wait_for_key:    u64,
  }
  pub struct SimpleTextOutput {
    reset:             u64,
    pub output_string: unsafe extern "efiapi" fn(*mut SimpleTextOutput, &u16),
    test_string:       u64,
    query_mode:        u64,
    set_mode:          u64,
    set_attr:          u64,
    pub clear_screen:  unsafe extern "efiapi" fn(*mut SimpleTextOutput),
    set_cursor_pos:    u64,
    enable_cursor:     u64,
    mode:              u64,
  }

  impl SimpleTextOutput {
    pub fn Print(&self, c: u16) {
      unsafe {
        //(self.output_string)(self, &c);
      }
    }
  }
}
