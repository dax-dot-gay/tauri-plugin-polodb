use serde_json::Value;
use tauri::{Manager, Runtime};

use crate::PolodbExt;

#[tauri::command]
pub async fn list_databases<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<Vec<String>, crate::Error> {
    app.polodb().list_databases().await
}

#[tauri::command]
pub async fn open_database<R: Runtime>(
    app: tauri::AppHandle<R>,
    key: String,
    path: String
) -> Result<String, crate::Error> {
    if let Ok(buf) = app.path().parse(path) {
        app.polodb().open_database(key, buf.to_str().unwrap().to_string()).await
    } else {
        Err(crate::Error::Io("Invalid path".to_string()))
    }
    
}

#[tauri::command]
pub async fn close_database<R: Runtime>(
    app: tauri::AppHandle<R>,
    key: String
) -> Result<String, crate::Error> {
    app.polodb().close_database(key).await
}

#[tauri::command]
pub async fn insert_document<R: Runtime>(
    app: tauri::AppHandle<R>,
    database: String,
    collection: String,
    documents: Vec<Value>
) -> Result<Vec<usize>, crate::Error> {
    app.polodb().insert(database, collection, documents).await
}
