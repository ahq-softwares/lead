use std::process::Command;

pub async fn postinstall(path: &str) {
  Command::new("chmod")
    .args(["777", path])
    .spawn()
    .unwrap()
    .wait()
    .unwrap();

  println!("Add {:?} to your PATH environment variable", &path);
}