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
    path: String,
) -> Result<String, crate::Error> {
    if let Ok(buf) = app.path().parse(path) {
        app.polodb()
            .open_database(key, buf.to_str().unwrap().to_string())
            .await
    } else {
        Err(crate::Error::Io("Invalid path".to_string()))
    }
}

#[tauri::command]
pub async fn close_database<R: Runtime>(
    app: tauri::AppHandle<R>,
    key: String,
) -> Result<String, crate::Error> {
    app.polodb().close_database(key).await
}

#[tauri::command]
pub async fn insert<R: Runtime>(
    app: tauri::AppHandle<R>,
    database: String,
    collection: String,
    documents: Vec<Value>,
) -> Result<Vec<usize>, crate::Error> {
    app.polodb().insert(database, collection, documents).await
}

#[tauri::command]
pub async fn insert_one<R: Runtime>(
    app: tauri::AppHandle<R>,
    database: String,
    collection: String,
    document: Value,
) -> Result<Vec<usize>, crate::Error> {
    app.polodb()
        .insert(database, collection, vec![document])
        .await
}

#[tauri::command]
pub async fn find<R: Runtime>(
    app: tauri::AppHandle<R>,
    database: String,
    collection: String,
    query: Value,
    sort: Option<Value>,
) -> Result<Vec<Value>, crate::Error> {
    match sort {
        Some(sorting) => {
            app.polodb()
                .find_sorted(database, collection, query, sorting)
                .await
        }
        None => app.polodb().find(database, collection, query).await,
    }
}

#[tauri::command]
pub async fn find_all<R: Runtime>(
    app: tauri::AppHandle<R>,
    database: String,
    collection: String,
    sort: Option<Value>,
) -> Result<Vec<Value>, crate::Error> {
    match sort {
        Some(sorting) => app.polodb().all_sorted(database, collection, sorting).await,
        None => app.polodb().all(database, collection).await,
    }
}

#[tauri::command]
pub async fn find_one<R: Runtime>(
    app: tauri::AppHandle<R>,
    database: String,
    collection: String,
    query: Value,
) -> Result<Value, crate::Error> {
    app.polodb().find_one(database, collection, query).await
}

#[tauri::command]
pub async fn delete<R: Runtime>(
    app: tauri::AppHandle<R>,
    database: String,
    collection: String,
    query: Value,
) -> Result<u64, crate::Error> {
    app.polodb().delete(database, collection, query).await
}

#[tauri::command]
pub async fn delete_one<R: Runtime>(
    app: tauri::AppHandle<R>,
    database: String,
    collection: String,
    query: Value,
) -> Result<u64, crate::Error> {
    app.polodb().delete_one(database, collection, query).await
}

#[tauri::command]
pub async fn delete_all<R: Runtime>(
    app: tauri::AppHandle<R>,
    database: String,
    collection: String
) -> Result<u64, crate::Error> {
    app.polodb().delete_all(database, collection).await
}

#[tauri::command]
pub async fn update<R: Runtime>(
    app: tauri::AppHandle<R>,
    database: String,
    collection: String,
    query: Value,
    update: Value,
    upsert: bool
) -> Result<u64, crate::Error> {
    app.polodb().update(database, collection, query, update, upsert).await
}

#[tauri::command]
pub async fn update_one<R: Runtime>(
    app: tauri::AppHandle<R>,
    database: String,
    collection: String,
    query: Value,
    update: Value,
    upsert: bool
) -> Result<u64, crate::Error> {
    app.polodb().update_one(database, collection, query, update, upsert).await
}

#[tauri::command]
pub async fn update_all<R: Runtime>(
    app: tauri::AppHandle<R>,
    database: String,
    collection: String,
    update: Value,
    upsert: bool
) -> Result<u64, crate::Error> {
    app.polodb().update_all(database, collection, update, upsert).await
}
