#[derive(Debug)]
pub struct KvError {

}

pub type Result<T> = std::result::Result<T, KvError>;