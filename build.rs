const COMMANDS: &[&str] = &["list_databases", "open_database", "close_database", "insert_document"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .build();
}
