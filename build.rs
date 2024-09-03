const COMMANDS: &[&str] = &["list-databases", "open-database", "close-database"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .build();
}
