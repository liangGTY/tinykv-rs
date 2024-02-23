use std::ptr::null;
use crate::config::Config;
use crate::storage::modify::Modify;
use crate::storage::{Storage, StorageReader};

pub struct StandaloneStorage {}

impl StandaloneStorage {
    pub fn new(conf: Config) -> StandaloneStorage {
        StandaloneStorage {}
    }
}

impl Storage for StandaloneStorage {
    fn start(&self) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }

    fn write(&self, batch: Vec<Modify>) {
        todo!()
    }

    fn reader(&self) -> Box<dyn StorageReader> {
        todo!()
    }
}