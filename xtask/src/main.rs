mod build_workspace;

fn print_help() {
  #[rustfmt::skip]
  eprintln!(
"Usage: cargo xtask <command>

Commands:
  a, all    Build and run the project [default]
  b, build  Build the project
  r, run    Run the project
  c, clean  Clean the project
  h, help   Print this help message
");
}

fn main() {
  match std::env::args().nth(1).as_deref() {
    Some("a") | Some("all") | None => {
      build();
      run()
    }
    Some("b") | Some("build") => build(),
    Some("r") | Some("run") => run(),
    Some("c") | Some("clean") => clean(),
    Some("h") | Some("help") => print_help(),
    Some(_) => {
      eprintln!("[Error] Unknown command");
      print_help();
    }
  }
}

fn workspace_root_dir() -> std::path::PathBuf {
  std::path::Path::new(&env!("CARGO_MANIFEST_DIR")) // "xtask/"
    .join("..")
    .canonicalize()
    .expect("Failed to canonicalize img directory path")
}
fn img_dir() -> std::path::PathBuf {
  const IMG_DIR_NAME: &'static str = "img";
  workspace_root_dir().join(IMG_DIR_NAME)
}

fn clean() {
  let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
  std::process::Command::new(cargo)
    .args(["clean", "--workspace"])
    .current_dir(workspace_root_dir())
    .status()
    .expect("Failed to clean workspace");

  if img_dir().exists() {
    std::fs::remove_dir_all(img_dir())
      .unwrap_or_else(|e| panic!("Failed to remove img directory: {}", e));
  }
}

fn build() {
  let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
  let workspace_dir = workspace_root_dir();
  let img_dir = img_dir();

  // without scope, compiler cannot assume that variables such as `cargo` live longer than threads
  std::thread::scope(|s| {
    s.spawn(|| {
      build_workspace::create_initfs_image(&cargo, &workspace_dir, &img_dir).map_or_else(
        |e| println!("Failed to build apps: {:?}", e),
        |_| println!(">> build finished: apps"),
      )
    });
    s.spawn(|| {
      build_workspace::build_kernel(&cargo, &workspace_dir, &img_dir).map_or_else(
        |e| println!("Failed to build kernel: {:?}", e),
        |_| println!(">> build finished: kernel"),
      )
    });
    s.spawn(|| {
      build_workspace::build_loader(&cargo, &workspace_dir, &img_dir).map_or_else(
        |e| println!("Failed to build loader: {:?}", e),
        |_| println!(">> build finished: loader"),
      )
    });
  });
}

fn run() {
  const OVMF_DIR: &'static str = "/usr/share/edk2/x64";

  let mut com = std::process::Command::new("qemu-system-x86_64");
  com.args([
    "-s",
    "-nographic",
    "-m",
    "2G",
    "-drive",
    &format!("if=pflash,format=raw,readonly=on,file={OVMF_DIR}/OVMF_CODE.4m.fd",),
    "-drive",
    &format!("if=pflash,format=raw,file=OVMF_VARS.4m.fd"),
    "-drive",
    &format!("format=raw,file=fat:rw:{}", img_dir().to_string_lossy()),
  ]);
  eprintln!(
    "Running: {} {}",
    com.get_program().to_string_lossy(),
    com
      .get_args()
      .map(|s| s.to_string_lossy())
      .collect::<Vec<_>>()
      .join(" ")
  );
  com.status().expect("Failed to run QEMU");
}
