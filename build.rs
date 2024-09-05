const COMMANDS: &[&str] = &[
    "list_databases",
    "open_database",
    "close_database",
    "insert",
    "insert_one",
    "find",
    "find_all",
    "find_one",
    "delete",
    "delete_all",
    "delete_one",
    "update",
    "update_all",
    "update_one",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
