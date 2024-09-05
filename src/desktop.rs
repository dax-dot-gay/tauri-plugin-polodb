use polodb_core::bson::{doc, to_document};
use serde::{de::DeserializeOwned, Serialize};
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::daemon::messages::{CountSelect, PoloCommand, PoloManager};

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

    pub async fn find<
        Doc: Serialize + DeserializeOwned,
        Query: Serialize + DeserializeOwned,
        Db: AsRef<str>,
        Coll: AsRef<str>,
    >(
        &self,
        database: Db,
        collection: Coll,
        query: Query,
    ) -> Result<Vec<Doc>, crate::Error> {
        self.api
            .call::<Vec<Doc>>(PoloCommand::Find {
                database: database.as_ref().to_string(),
                collection: collection.as_ref().to_string(),
                query: to_document(&query).unwrap(),
                count: CountSelect::Many,
                sort: None,
            })
            .await
    }

    pub async fn find_sorted<
        Doc: Serialize + DeserializeOwned,
        Query: Serialize + DeserializeOwned,
        Sort: Serialize + DeserializeOwned,
        Db: AsRef<str>,
        Coll: AsRef<str>,
    >(
        &self,
        database: Db,
        collection: Coll,
        query: Query,
        sort: Sort,
    ) -> Result<Vec<Doc>, crate::Error> {
        self.api
            .call::<Vec<Doc>>(PoloCommand::Find {
                database: database.as_ref().to_string(),
                collection: collection.as_ref().to_string(),
                query: to_document(&query).unwrap(),
                count: CountSelect::Many,
                sort: Some(to_document(&sort).unwrap()),
            })
            .await
    }

    pub async fn find_one<
        Doc: Serialize + DeserializeOwned,
        Query: Serialize + DeserializeOwned,
        Db: AsRef<str>,
        Coll: AsRef<str>,
    >(
        &self,
        database: Db,
        collection: Coll,
        query: Query,
    ) -> Result<Doc, crate::Error> {
        self.api
            .call::<Doc>(PoloCommand::Find {
                database: database.as_ref().to_string(),
                collection: collection.as_ref().to_string(),
                query: to_document(&query).unwrap(),
                count: CountSelect::One,
                sort: None,
            })
            .await
    }

    pub async fn all<Doc: Serialize + DeserializeOwned, Db: AsRef<str>, Coll: AsRef<str>>(
        &self,
        database: Db,
        collection: Coll,
    ) -> Result<Vec<Doc>, crate::Error> {
        self.api
            .call::<Vec<Doc>>(PoloCommand::Find {
                database: database.as_ref().to_string(),
                collection: collection.as_ref().to_string(),
                query: doc! {},
                count: CountSelect::Many,
                sort: None,
            })
            .await
    }

    pub async fn all_sorted<
        Doc: Serialize + DeserializeOwned,
        Sort: Serialize + DeserializeOwned,
        Db: AsRef<str>,
        Coll: AsRef<str>,
    >(
        &self,
        database: Db,
        collection: Coll,
        sort: Sort,
    ) -> Result<Vec<Doc>, crate::Error> {
        self.api
            .call::<Vec<Doc>>(PoloCommand::Find {
                database: database.as_ref().to_string(),
                collection: collection.as_ref().to_string(),
                query: doc! {},
                count: CountSelect::Many,
                sort: Some(to_document(&sort).unwrap()),
            })
            .await
    }

    pub async fn delete<Query: Serialize + DeserializeOwned, Db: AsRef<str>, Coll: AsRef<str>>(
        &self,
        database: Db,
        collection: Coll,
        query: Query,
    ) -> Result<u64, crate::Error> {
        self.api
            .call::<u64>(PoloCommand::Delete {
                database: database.as_ref().to_string(),
                collection: collection.as_ref().to_string(),
                query: to_document(&query).unwrap(),
                count: CountSelect::Many,
            })
            .await
    }

    pub async fn delete_one<
        Query: Serialize + DeserializeOwned,
        Db: AsRef<str>,
        Coll: AsRef<str>,
    >(
        &self,
        database: Db,
        collection: Coll,
        query: Query,
    ) -> Result<u64, crate::Error> {
        self.api
            .call::<u64>(PoloCommand::Delete {
                database: database.as_ref().to_string(),
                collection: collection.as_ref().to_string(),
                query: to_document(&query).unwrap(),
                count: CountSelect::One,
            })
            .await
    }

    pub async fn delete_all<Db: AsRef<str>, Coll: AsRef<str>>(
        &self,
        database: Db,
        collection: Coll,
    ) -> Result<u64, crate::Error> {
        self.api
            .call::<u64>(PoloCommand::Delete {
                database: database.as_ref().to_string(),
                collection: collection.as_ref().to_string(),
                query: doc! {},
                count: CountSelect::Many,
            })
            .await
    }

    pub async fn update<
        Query: Serialize + DeserializeOwned,
        Update: Serialize + DeserializeOwned,
        Db: AsRef<str>,
        Coll: AsRef<str>,
    >(
        &self,
        database: Db,
        collection: Coll,
        query: Query,
        update: Update,
        upsert: bool
    ) -> Result<u64, crate::Error> {
        self.api
            .call::<u64>(PoloCommand::Update {
                database: database.as_ref().to_string(),
                collection: collection.as_ref().to_string(),
                query: to_document(&query).unwrap(),
                update: to_document(&update).unwrap(),
                upsert,
                count: CountSelect::Many,
            })
            .await
    }

    pub async fn update_one<
        Query: Serialize + DeserializeOwned,
        Update: Serialize + DeserializeOwned,
        Db: AsRef<str>,
        Coll: AsRef<str>,
    >(
        &self,
        database: Db,
        collection: Coll,
        query: Query,
        update: Update,
        upsert: bool
    ) -> Result<u64, crate::Error> {
        self.api
            .call::<u64>(PoloCommand::Update {
                database: database.as_ref().to_string(),
                collection: collection.as_ref().to_string(),
                query: to_document(&query).unwrap(),
                update: to_document(&update).unwrap(),
                upsert,
                count: CountSelect::One,
            })
            .await
    }

    pub async fn update_all<
        Update: Serialize + DeserializeOwned,
        Db: AsRef<str>,
        Coll: AsRef<str>,
    >(
        &self,
        database: Db,
        collection: Coll,
        update: Update,
        upsert: bool
    ) -> Result<u64, crate::Error> {
        self.api
            .call::<u64>(PoloCommand::Update {
                database: database.as_ref().to_string(),
                collection: collection.as_ref().to_string(),
                query: doc! {},
                update: to_document(&update).unwrap(),
                upsert,
                count: CountSelect::Many,
            })
            .await
    }
}
