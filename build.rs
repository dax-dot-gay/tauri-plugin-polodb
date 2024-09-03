const COMMANDS: &[&str] = &["list-databases"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .build();
}
