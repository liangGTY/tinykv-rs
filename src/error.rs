use std::io;
use std::io::Error;

#[derive(Debug)]
pub enum KvError {
    Io(io::Error),
    SerdeError(serde_json::Error),
    KeyNotFound,
    UnexpectedCommandType
}

impl From<io::Error> for KvError {
    fn from(value: Error) -> Self {
        KvError::Io(value)
    }
}

impl From<serde_json::Error> for KvError {
    fn from(value: serde_json::Error) -> Self {
        KvError::SerdeError(value)
    }
}

pub type Result<T> = std::result::Result<T, KvError>;