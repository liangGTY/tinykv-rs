pub struct Modify {
    data: Command,
}

enum Command {
    Put {
        key: Vec<u8>,
        value: Vec<u8>,
        cf: String,
    },
    Delete {
        key: Vec<u8>,
        cf: String,
    },
}

impl Modify {
    fn key(&self) -> Vec<u8> {
        return match &self.data {
            Command::Put { key, .. } => { key.to_vec() }
            Command::Delete { key, .. } => { key.to_vec() }
        };
    }

    fn value(&self) -> Option<Vec<u8>> {
        return match &self.data {
            Command::Put { value, .. } => { Some(value.to_vec()) }
            Command::Delete { .. } => { None }
        };
    }
    fn cf(&self) -> String {
        return match &self.data {
            Command::Put { cf, .. } => { cf.into() }
            Command::Delete {cf, .. } => { cf.into() }
        };
    }
}