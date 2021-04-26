use std::collections::HashMap;

use crate::enums::HandleResult;
use crate::store::Store;

/// Simple store does not support transactions but does support READ/WRITE/DELETE
pub struct SimpleStore {
    store: HashMap<String, String>
}

impl SimpleStore {
    pub fn new() -> SimpleStore {
        let store = HashMap::new();
        SimpleStore{ store }
    }
}

impl Store for SimpleStore {
    fn read(&self, key: String) -> HandleResult {
        match self.store.get(&key) {
            None => HandleResult::Failure(format!("Key not found: {}", key)),
            Some(value) => HandleResult::Result(value.to_owned())
        }
    }

    fn write(&mut self, key: String, value: String) -> HandleResult {
        self.store.insert(key, value);
        HandleResult::Success
    }

    fn delete(&mut self, key: String) -> HandleResult {
        self.store.remove(&key);
        HandleResult::Success
    }

    fn start(&mut self) -> HandleResult {
        HandleResult::Failure("Transactions not supported by this backend".to_owned())
    }

    fn abort(&mut self) -> HandleResult {
        HandleResult::Failure("Transactions not supported by this backend".to_owned())
    }

    fn commit(&mut self) -> HandleResult {
        HandleResult::Failure("Transactions not supported by this backend".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let mut store = SimpleStore::new();
        store.write("foo".to_owned(), "bar".to_owned());
        assert_eq!(HandleResult::Result("bar".to_owned()), store.read("foo".to_owned()));
    }
}