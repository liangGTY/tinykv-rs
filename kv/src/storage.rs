use crate::storage::modify::Modify;

mod modify;
pub mod standalone_storage;

pub trait Storage {
    fn start(&self);

    fn stop(&self);

    fn write(&self, batch: Vec<Modify>);

    fn reader(&self) -> Box<dyn StorageReader>;
}

trait StorageReader {}
