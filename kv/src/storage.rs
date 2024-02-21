use crate::storage::modify::Modify;

mod modify;
mod standalone_storage;

pub trait Storage {
    fn start();

    fn stop();

    fn write(batch: Vec<Modify>);

    fn reader() -> impl StorageReader;
}

trait StorageReader{}
