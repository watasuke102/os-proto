pub(super) fn build_kernel(
  cargo: &String,
  workspace_root: &std::path::PathBuf,
  img_dir: &std::path::PathBuf,
) -> Result<(), String> {
  std::process::Command::new(cargo)
    .args([
      "build",
      "--quiet",
      "--package=kernel",
      "-Zjson-target-spec",
      "--target",
      &workspace_root
        .join("kernel/x86_64-unknown-os.json")
        .to_string_lossy(),
      "-Zbuild-std=core,alloc,compiler_builtins",
      "-Zbuild-std-features=compiler-builtins-mem",
      "-Zunstable-options",
      "--artifact-dir",
      &img_dir.to_string_lossy(),
    ])
    .current_dir(workspace_root)
    .status()
    .map(|_| ())
    .map_err(|e| e.to_string())
}

pub(super) fn build_loader(
  cargo: &String,
  workspace_root: &std::path::PathBuf,
  img_dir: &std::path::PathBuf,
) -> Result<(), String> {
  std::process::Command::new(cargo)
    .args([
      "build",
      "--quiet",
      "--package=loader",
      "--target=x86_64-unknown-uefi",
      "-Zbuild-std=core,alloc,compiler_builtins",
      "-Zbuild-std-features=compiler-builtins-mem",
    ])
    .current_dir(workspace_root)
    .status()
    .map_err(|e| e.to_string())?;

  let loader_dest_dir = img_dir.join("EFI/BOOT");
  std::fs::create_dir_all(&loader_dest_dir).map_err(|e| e.to_string())?;
  std::fs::copy(
    workspace_root.join("target/x86_64-unknown-uefi/debug/loader.efi"),
    loader_dest_dir.join("BOOTX64.EFI"),
  )
  .map(|_| ())
  .map_err(|e| e.to_string())
}

pub(super) fn create_initfs_image(
  cargo: &String,
  workspace_root: &std::path::PathBuf,
  img_dir: &std::path::PathBuf,
) -> Result<(), String> {
  std::process::Command::new(cargo)
    .args([
      "build",
      "--quiet",
      "--package=apps",
      "--target",
      "x86_64-unknown-none",
    ])
    .env("RUSTFLAGS", "-C link-arg=--image-base=0xffff800000000000")
    .current_dir(workspace_root)
    .status()
    .map_err(|e| e.to_string())?;

  let apps_dir = workspace_root.join("apps/src/bin");
  let apps_files = std::fs::read_dir(&apps_dir)
    .map_err(|e| e.to_string())?
    .filter_map(|entry| {
      entry.ok().and_then(|e| {
        let path = e.path();
        if path.extension()?.to_str()? == "rs" {
          Some(
            workspace_root
              .join("target/x86_64-unknown-none/debug")
              .join(path.file_stem()?) // exclude .rs extension
              .to_string_lossy()
              .to_string(),
          )
        } else {
          None
        }
      })
    })
    .collect::<Vec<_>>();

  let initfs_img_path = img_dir.join("initfs.img");
  let initfs_img_path = initfs_img_path.to_string_lossy();
  std::process::Command::new("qemu-img")
    .args(["create", "-f", "raw", &initfs_img_path, "8M"])
    .status()
    .map_err(|e| e.to_string())?;
  std::process::Command::new("mkfs.fat")
    .args([
      "-n", // volume name
      "'INITFS'",
      "-s2",  // sector size
      "-f2",  // sectors per cluster
      "-R32", // reserved sectors
      "-F32", // FAT32
      &initfs_img_path,
    ])
    .status()
    .map_err(|e| e.to_string())?;
  std::process::Command::new("mcopy")
    .args(["-i", &initfs_img_path])
    .args(apps_files)
    .arg("::") // copy to root of the image
    .status()
    .map(|_| ())
    .map_err(|e| e.to_string())
}
