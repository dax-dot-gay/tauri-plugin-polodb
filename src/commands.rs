use tauri::Runtime;

use crate::{daemon::messages::PoloCommand, PolodbExt};

#[tauri::command]
pub async fn list_databases<R: Runtime>(
    app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<Vec<String>, crate::Error> {
    app.polodb()
        .call::<Vec<String>>(PoloCommand::ListDatabases)
        .await
}
