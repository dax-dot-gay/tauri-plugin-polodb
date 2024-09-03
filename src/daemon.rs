use std::{collections::HashMap, path::Path, sync::{Arc, Mutex, MutexGuard}};

use polodb_core::{Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SerializedDatabase {
    pub key: String,
    pub file: String
}

pub struct PoloDatabase {
    pub key: String,
    pub database: Database,
    pub file: String
}

impl PoloDatabase {
    pub fn deserialize(serialized: SerializedDatabase) -> Result<Self, crate::Error> {
        let db = Database::open_path(Path::new(serialized.file.as_str())).or_else(|e| Err(crate::Error::Io(format!("Failed to open {:?}: {:?}", serialized.file.as_str(), e))))?;
        Ok(PoloDatabase {
            key: serialized.key,
            database: db,
            file: serialized.file
        })
    }

    pub fn serialize(&self) -> SerializedDatabase {
        SerializedDatabase {
            key: self.key.clone(),
            file: self.file.clone()
        }
    }

    pub fn collection<T: Serialize, S: AsRef<str>>(&self, name: S) -> Collection<T> {
        self.database.collection::<T>(name.as_ref())
    }

    pub fn collections(&self) -> Result<Vec<String>, crate::Error> {
        self.database.list_collection_names().or(Err(crate::Error::DatabaseError("Failed to list collections".to_string())))
    }
}

pub mod messages {
    use std::{sync::{Arc, Mutex}, thread::{spawn, JoinHandle}};

    use async_channel::{unbounded, Receiver, Sender};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum PoloCommand {
        Kill
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum PoloReturn {
        UnknownCommand,
        Error(crate::Error)
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum PoloMessageContent {
        Command(PoloCommand),
        Return(PoloReturn)
    }


    #[derive(Clone, Debug)]
    pub struct PoloMessage {
        id: Uuid,
        content: PoloMessageContent,
        return_pipe: Sender<PoloMessage>
    }

    #[derive(Clone, Debug)]
    pub struct PoloManager {
        handle: Arc<Mutex<JoinHandle<()>>>,
        tx: Sender<PoloMessage>
    }

    impl PoloManager {
        fn daemon(rx: Receiver<PoloMessage>) -> () {
            loop {
                if let Ok(msg) = rx.recv_blocking() {
                    if let PoloMessageContent::Command(command) = msg.content {
                        #[allow(unreachable_patterns)]
                        let result = match command {
                            PoloCommand::Kill => break,
                            _ => PoloReturn::UnknownCommand
                        };

                        let _ = msg.return_pipe.send(PoloMessage {id: msg.id, content: PoloMessageContent::Return(result), return_pipe: msg.return_pipe.clone()});
                    }
                }
            }
        }

        pub fn new() -> Self {
            let (tx, rx) = unbounded::<PoloMessage>();
            let handle = spawn(move || {
                PoloManager::daemon(rx)
            });
            PoloManager {
                handle: Arc::new(Mutex::new(handle)),
                tx: tx.clone()
            }
        }

        pub async fn call(&self, command: PoloCommand) -> Result<PoloReturn, crate::Error> {
            let (tx, rx) = unbounded::<PoloMessage>();
            let id = Uuid::new_v4();
            let message = PoloMessage {
                id: id.clone(),
                content: PoloMessageContent::Command(command.clone()),
                return_pipe: tx.clone()
            };
            self.tx.send(message.clone()).await.or(Err(crate::Error::DaemonError("Channel send failure".to_string())))?;
            match rx.recv().await {
                Ok(result) => match result.content {
                    PoloMessageContent::Command(_) => Err(crate::Error::DaemonError("Critical recv error: command in return channel.".to_string())),
                    PoloMessageContent::Return(r) => Ok(r)
                },
                Err(_) => Err(crate::Error::DaemonError("Failed to recv daemon response".to_string()))
            }
        }

        pub async fn call_nowait(&self, command: PoloCommand) -> Result<(), crate::Error> {
            let (tx, _) = unbounded::<PoloMessage>();
            let id = Uuid::new_v4();
            let message = PoloMessage {
                id: id.clone(),
                content: PoloMessageContent::Command(command.clone()),
                return_pipe: tx.clone()
            };
            self.tx.send(message.clone()).await.or(Err(crate::Error::DaemonError("Channel send failure".to_string())))?;
            Ok(())
        }

        pub async fn kill(&self) -> Result<(), crate::Error> {
            self.call_nowait(PoloCommand::Kill).await?;
            self.handle.lock().or(Err(crate::Error::DaemonError("Handle lock failed".to_string()))).and(Ok(()))
        }
    }
}

pub struct PoloDaemon {
    pub databases: HashMap<String, Arc<Mutex<PoloDatabase>>>
}

impl PoloDaemon {
    pub fn new() -> Self {
        PoloDaemon {
            databases: HashMap::new()
        }
    }

    pub fn get<K: AsRef<str>>(&self, key: K) -> Result<MutexGuard<'_, PoloDatabase>, crate::Error> {
        match self.databases.get(key.as_ref()) {
            Some(arc) => arc.lock().or(Err(crate::Error::Sync("daemon.get".to_string()))),
            None => Err(crate::Error::UnknownDatabase(key.as_ref().to_string()))
        }
    }

    pub fn open<K: AsRef<str>, F: AsRef<Path>>(&mut self, key: K, path: F) -> Result<(), crate::Error> {
        if self.databases.contains_key(key.as_ref()) {
            return Err(crate::Error::ExistingDatabase(key.as_ref().to_string()));
        }
        let path_string = path.as_ref().to_str().unwrap().to_string();
        let db = Database::open_path(path.as_ref()).or_else(|e| Err(crate::Error::Io(format!("Failed to open {:?}: {:?}", path_string.clone(), e))))?;
        self.databases.insert(key.as_ref().to_string(), Arc::new(Mutex::new(PoloDatabase {
            key: key.as_ref().to_string(),
            database: db,
            file: path_string.clone()
        })));
        Ok(())
    }

    pub fn close<K: AsRef<str>>(&mut self, key: K) -> Result<(), crate::Error> {
        match self.databases.remove(key.as_ref()) {
            Some(_) => Ok(()),
            None => Err(crate::Error::UnknownDatabase(key.as_ref().to_string()))
        }
    }

    pub fn list(&self) -> Vec<String> {
        self.databases.keys().map(|s| s.clone()).collect()
    }
}