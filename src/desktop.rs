use polodb_core::bson::to_document;
use serde::{de::DeserializeOwned, Serialize};
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::daemon::messages::{PoloCommand, PoloManager};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<Polodb<R>, ()> {
    Ok(Polodb {
        app: app.clone(),
        api: PoloManager::new(),
    })
}

/// Access to the polodb APIs.
pub struct Polodb<R: Runtime> {
    pub app: AppHandle<R>,
    pub api: PoloManager,
}

impl<R: Runtime> Polodb<R> {
    pub async fn call<T: Serialize + DeserializeOwned>(
        &self,
        command: PoloCommand,
    ) -> Result<T, crate::Error> {
        self.api.call::<T>(command).await
    }

    pub async fn open_database<T: AsRef<str>, P: AsRef<str>>(
        &self,
        key: T,
        path: P,
    ) -> Result<String, crate::Error> {
        self.api
            .call::<String>(PoloCommand::OpenDatabase {
                key: key.as_ref().to_string(),
                path: path.as_ref().to_string(),
            })
            .await
    }

    pub async fn close_database<T: AsRef<str>>(&self, key: T) -> Result<String, crate::Error> {
        self.api
            .call::<String>(PoloCommand::CloseDatabase(key.as_ref().to_string()))
            .await
    }

    pub async fn list_databases(&self) -> Result<Vec<String>, crate::Error> {
        self.api
            .call::<Vec<String>>(PoloCommand::ListDatabases)
            .await
    }

    pub async fn insert<Doc: Serialize + DeserializeOwned, Db: AsRef<str>, Coll: AsRef<str>>(
        &self,
        database: Db,
        collection: Coll,
        documents: Vec<Doc>,
    ) -> Result<Vec<usize>, crate::Error> {
        self.api
            .call::<Vec<usize>>(PoloCommand::Insert {
                database: database.as_ref().to_string(),
                collection: collection.as_ref().to_string(),
                value: documents.iter().map(|d| to_document(d).unwrap()).collect(),
            })
            .await
    }
}
