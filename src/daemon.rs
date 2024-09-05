use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex, MutexGuard},
};

use polodb_core::{bson::Document, Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SerializedDatabase {
    pub key: String,
    pub file: String,
}

pub struct PoloDatabase {
    pub key: String,
    pub database: Database,
    pub file: String,
}

impl PoloDatabase {
    pub fn deserialize(serialized: SerializedDatabase) -> Result<Self, crate::Error> {
        let db = Database::open_path(Path::new(serialized.file.as_str())).or_else(|e| {
            Err(crate::Error::Io(format!(
                "Failed to open {:?}: {:?}",
                serialized.file.as_str(),
                e
            )))
        })?;
        Ok(PoloDatabase {
            key: serialized.key,
            database: db,
            file: serialized.file,
        })
    }

    pub fn serialize(&self) -> SerializedDatabase {
        SerializedDatabase {
            key: self.key.clone(),
            file: self.file.clone(),
        }
    }

    pub fn collection<T: Serialize, S: AsRef<str>>(&self, name: S) -> Collection<T> {
        self.database.collection::<T>(name.as_ref())
    }

    pub fn collections(&self) -> Result<Vec<String>, crate::Error> {
        self.database
            .list_collection_names()
            .or(Err(crate::Error::DatabaseError(
                "Failed to list collections".to_string(),
            )))
    }
}

pub mod messages {
    use std::{
        path::Path,
        sync::{Arc, Mutex},
        thread::{spawn, JoinHandle},
    };

    use async_channel::{unbounded, Receiver, Sender};
    use polodb_core::{bson::Document, options::UpdateOptions, CollectionT};
    use serde::{de::DeserializeOwned, Deserialize, Serialize};
    use serde_json::Value;
    use uuid::Uuid;

    use super::PoloDaemon;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum CountSelect {
        One,
        Many,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum PoloCommand {
        Kill,
        OpenDatabase {
            key: String,
            path: String,
        },
        CloseDatabase(String),
        ListDatabases,
        Insert {
            database: String,
            collection: String,
            value: Vec<Document>,
        },
        Delete {
            database: String,
            collection: String,
            query: Document,
            count: CountSelect,
        },
        Update {
            database: String,
            collection: String,
            query: Document,
            update: Document,
            count: CountSelect,
            upsert: bool
        },
        Find {
            database: String,
            collection: String,
            query: Document,
            count: CountSelect,
            sort: Option<Document>,
        },
    }

    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    pub struct PoloMessage {
        id: Uuid,
        content: PoloCommand,
        return_pipe: Sender<Result<Value, crate::Error>>,
    }

    impl PoloMessage {
        pub fn respond<T: Serialize>(&self, data: Result<T, crate::Error>) -> () {
            let _ = self.return_pipe.send_blocking(data.and_then(|d| {
                serde_json::to_value(d).or(Err(crate::Error::SerializationError(
                    "Response serialization failure".to_string(),
                )))
            }));
        }
    }

    #[derive(Clone, Debug)]
    pub struct PoloManager {
        handle: Arc<Mutex<JoinHandle<()>>>,
        tx: Sender<PoloMessage>,
    }

    impl PoloManager {
        fn daemon(rx: Receiver<PoloMessage>) -> () {
            #[allow(unused_variables, unused_mut)]
            let mut daemon = PoloDaemon::new();
            loop {
                if let Ok(msg) = rx.recv_blocking() {
                    let command = msg.clone().content;
                    #[allow(unreachable_patterns)]
                    match command {
                        PoloCommand::Kill => break,
                        PoloCommand::OpenDatabase { key, path } => {
                            msg.respond(match daemon.open(key, Path::new(path.as_str())) {
                                Ok(_) => Ok("Database opened.".to_string()),
                                Err(e) => Err(e),
                            })
                        }
                        PoloCommand::CloseDatabase(key) => msg.respond(match daemon.close(key) {
                            Ok(_) => Ok("Database closed.".to_string()),
                            Err(e) => Err(e),
                        }),
                        PoloCommand::ListDatabases => msg.respond(Ok(daemon.list().clone())),
                        PoloCommand::Insert {
                            database,
                            collection,
                            value,
                        } => {
                            msg.respond(daemon.get_collection(database, collection).and_then(|c| {
                                c.insert_many(value)
                                    .and_then(|r| {
                                        Ok(r.inserted_ids
                                            .keys()
                                            .map(|i| i.clone())
                                            .collect::<Vec<usize>>())
                                    })
                                    .or_else(|e| Err(crate::Error::InsertError(e.to_string())))
                            }))
                        }
                        PoloCommand::Find {
                            database,
                            collection,
                            query,
                            count,
                            sort,
                        } => {
                            msg.respond(daemon.get_collection(database, collection).and_then(|c| {
                                match count {
                                    CountSelect::Many => match sort {
                                        Some(sorting) => c
                                            .find(query)
                                            .sort(sorting)
                                            .run()
                                            .and_then(|s| {
                                                Ok(s.filter_map(|d| match d {
                                                    Ok(v) => Some(v),
                                                    Err(_) => None,
                                                })
                                                .collect())
                                            })
                                            .or(Err(crate::Error::DatabaseError(
                                                "Result collection failed".to_string(),
                                            ))),
                                        None => c
                                            .find(query)
                                            .run()
                                            .and_then(|s| {
                                                Ok(s.filter_map(|d| match d {
                                                    Ok(v) => Some(v),
                                                    Err(_) => None,
                                                })
                                                .collect())
                                            })
                                            .or(Err(crate::Error::DatabaseError(
                                                "Result collection failed".to_string(),
                                            ))),
                                    },
                                    CountSelect::One => c
                                        .find_one(query)
                                        .and_then(|v| match v {
                                            Some(d) => Ok(vec![d]),
                                            None => Err(polodb_core::Error::UnexpectedPageType),
                                        })
                                        .or(Err(crate::Error::DatabaseError(
                                            "Failed to find document".to_string(),
                                        ))),
                                }
                            }))
                        }
                        PoloCommand::Delete {
                            database,
                            collection,
                            query,
                            count,
                        } => msg.respond(daemon.get_collection(database, collection).and_then(
                            |coll| {
                                match count {
                                    CountSelect::Many => coll
                                        .delete_many(query)
                                        .or(Err(crate::Error::DatabaseError(
                                            "Failed to delete specified documents".to_string(),
                                        )))
                                        .and_then(|r| Ok(r.deleted_count)),
                                    CountSelect::One => coll
                                        .delete_one(query)
                                        .or(Err(crate::Error::DatabaseError(
                                            "Failed to delete specified document".to_string(),
                                        )))
                                        .and_then(|r| Ok(r.deleted_count)),
                                }
                            },
                        )),
                        PoloCommand::Update {
                            database,
                            collection,
                            query,
                            update,
                            count,
                            upsert
                        } => msg.respond(daemon.get_collection(database, collection).and_then(
                            |coll| {
                                match count {
                                    CountSelect::Many => coll
                                        .update_many_with_options(query, update, UpdateOptions {upsert: Some(upsert)})
                                        .or(Err(crate::Error::DatabaseError(
                                            "Failed to update specified documents".to_string(),
                                        )))
                                        .and_then(|r| Ok(r.modified_count)),
                                    CountSelect::One => coll
                                        .update_one_with_options(query, update, UpdateOptions {upsert: Some(upsert)})
                                        .or(Err(crate::Error::DatabaseError(
                                            "Failed to update specified document".to_string(),
                                        )))
                                        .and_then(|r| Ok(r.modified_count)),
                                }
                            },
                        )),
                        _ => msg.respond::<()>(Err(crate::Error::DaemonError(
                            "Unknown command".to_string(),
                        ))),
                    };
                }
            }
        }

        pub fn new() -> Self {
            let (tx, rx) = unbounded::<PoloMessage>();
            let handle = spawn(move || PoloManager::daemon(rx));
            PoloManager {
                handle: Arc::new(Mutex::new(handle)),
                tx: tx.clone(),
            }
        }

        pub async fn call<T: Serialize + DeserializeOwned>(
            &self,
            command: PoloCommand,
        ) -> Result<T, crate::Error> {
            let (tx, rx) = unbounded::<Result<Value, crate::Error>>();
            let id = Uuid::new_v4();
            let message = PoloMessage {
                id: id.clone(),
                content: command.clone(),
                return_pipe: tx.clone(),
            };
            self.tx
                .send(message.clone())
                .await
                .or(Err(crate::Error::DaemonError(
                    "Channel send failure".to_string(),
                )))?;
            match rx.recv().await {
                Ok(result) => match result {
                    Ok(v) => {
                        serde_json::from_value::<T>(v).or(Err(crate::Error::SerializationError(
                            "Failed to deserialize reponse value".to_string(),
                        )))
                    }
                    Err(e) => Err(e),
                },
                Err(_) => Err(crate::Error::DaemonError(
                    "Failed to recv daemon response".to_string(),
                )),
            }
        }

        pub async fn call_nowait(&self, command: PoloCommand) -> Result<(), crate::Error> {
            let (tx, _) = unbounded::<Result<Value, crate::Error>>();
            let id = Uuid::new_v4();
            let message = PoloMessage {
                id: id.clone(),
                content: command.clone(),
                return_pipe: tx.clone(),
            };
            self.tx
                .send(message.clone())
                .await
                .or(Err(crate::Error::DaemonError(
                    "Channel send failure".to_string(),
                )))?;
            Ok(())
        }

        pub async fn kill(&self) -> Result<(), crate::Error> {
            self.call_nowait(PoloCommand::Kill).await?;
            self.handle
                .lock()
                .or(Err(crate::Error::DaemonError(
                    "Handle lock failed".to_string(),
                )))
                .and(Ok(()))
        }
    }
}

pub struct PoloDaemon {
    pub databases: HashMap<String, Arc<Mutex<PoloDatabase>>>,
}

impl PoloDaemon {
    pub fn new() -> Self {
        PoloDaemon {
            databases: HashMap::new(),
        }
    }

    pub fn get<K: AsRef<str>>(&self, key: K) -> Result<MutexGuard<'_, PoloDatabase>, crate::Error> {
        match self.databases.get(key.as_ref()) {
            Some(arc) => arc
                .lock()
                .or(Err(crate::Error::Sync("daemon.get".to_string()))),
            None => Err(crate::Error::UnknownDatabase(key.as_ref().to_string())),
        }
    }

    pub fn open<K: AsRef<str>, F: AsRef<Path>>(
        &mut self,
        key: K,
        path: F,
    ) -> Result<(), crate::Error> {
        if self.databases.contains_key(key.as_ref()) {
            return Err(crate::Error::ExistingDatabase(key.as_ref().to_string()));
        }
        let path_string = path.as_ref().to_str().unwrap().to_string();
        let db = Database::open_path(path.as_ref()).or_else(|e| {
            Err(crate::Error::Io(format!(
                "Failed to open {:?}: {:?}",
                path_string.clone(),
                e
            )))
        })?;
        self.databases.insert(
            key.as_ref().to_string(),
            Arc::new(Mutex::new(PoloDatabase {
                key: key.as_ref().to_string(),
                database: db,
                file: path_string.clone(),
            })),
        );
        Ok(())
    }

    pub fn close<K: AsRef<str>>(&mut self, key: K) -> Result<(), crate::Error> {
        match self.databases.remove(key.as_ref()) {
            Some(_) => Ok(()),
            None => Err(crate::Error::UnknownDatabase(key.as_ref().to_string())),
        }
    }

    pub fn list(&self) -> Vec<String> {
        self.databases.keys().map(|s| s.clone()).collect()
    }

    pub fn get_collection(
        &self,
        database: String,
        collection: String,
    ) -> Result<Collection<Document>, crate::Error> {
        let db = match self.databases.get(&database) {
            Some(locked) => locked.lock().or(Err(crate::Error::Sync(
                "Failed to acquire DB lock".to_string(),
            ))),
            None => Err(crate::Error::UnknownDatabase("Invalid DB key".to_string())),
        }?;
        Ok(db.collection::<Document, String>(collection))
    }
}
