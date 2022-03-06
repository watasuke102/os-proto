use std::{env, path::Path, process::Command};

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();

  Command::new("nasm")
    .args(&[
      "-f",
      "elf64",
      "-o",
      &format!("{}/entry.o", out_dir),
      "src/entry.asm",
    ])
    .status()
    .unwrap();
  Command::new("ar")
    .args(&["crus", "libentry.a", "entry.o"])
    .current_dir(&Path::new(&out_dir))
    .status()
    .unwrap();

  println!("cargo:rustc-link-search=native={}", out_dir);
  println!("cargo:rustc-link-lib=static=entry");
  println!("cargo:rerun-if-changed=src/entry.asm");
}
