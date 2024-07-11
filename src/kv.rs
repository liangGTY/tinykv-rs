use std::collections::HashMap;
use std::path::Path;
use super::Result;

pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<Self> {
        todo!()
    }

    pub fn set(&mut self, key: String, value: String) ->Result<()> {
        self.map.insert(key, value);
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.map.get(&key).cloned();
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
