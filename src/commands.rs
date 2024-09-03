use tauri::Runtime;

use crate::PolodbExt;

#[tauri::command]
pub async fn list_databases<R: Runtime>(
    app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<Vec<String>, crate::Error> {
    app.polodb().list_databases().await
}

#[tauri::command]
pub async fn open_database<R: Runtime>(
    app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    key: String,
    path: String
) -> Result<String, crate::Error> {
    app.polodb().open_database(key, path).await
}

#[tauri::command]
pub async fn close_database<R: Runtime>(
    app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    key: String
) -> Result<String, crate::Error> {
    app.polodb().close_database(key).await
}
