use crate::storage::Storage;

pub struct Server {
    storage: Box<dyn Storage>,
}

impl Server {
    pub fn new(storage: Box<dyn Storage>) -> Server {
        Server {
            storage
        }
    }
}