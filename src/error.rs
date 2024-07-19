use std::io;
use std::io::Error;

#[derive(Debug)]
pub enum KvError {
    Io(io::Error),
}

impl From<io::Error> for KvError {
    fn from(value: Error) -> Self {
        KvError::Io(value)
    }
}

pub type Result<T> = std::result::Result<T, KvError>;