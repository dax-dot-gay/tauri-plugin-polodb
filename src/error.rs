use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Error {
    Sync(String),
    UnknownDatabase(String),
    Io(String),
    ExistingDatabase(String),
    DatabaseError(String),
    DaemonError(String),
    SerializationError(String),
    InsertError(String)
}