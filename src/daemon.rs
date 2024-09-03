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