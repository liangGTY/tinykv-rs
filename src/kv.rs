use std::collections::{BTreeMap, HashMap};
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::Path;
use crate::kv::Command::{Remove, Set};
use super::Result;

pub struct KvStore {
    map: HashMap<String, String>,
    index: BTreeMap<String, CommandPos>,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<Self> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(path)?;

        let writer = BufWriter::new(file);
        let mut index = BTreeMap::new();
        todo!()
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::set(key, value);

        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.map.get(&key).cloned();
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}

struct CommandPos {
    gen: u64,
    pos: u64,
    len: u64,
}

enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {

    fn set(key: String, value: String) -> Command {
        Set { key, value }
    }

    fn remove(key: String, value: String) -> Command {
        Remove { key }
    }
}