use std::collections::HashMap;
use std::vec::Vec;

use crate::enums::HandleResult;
use crate::store::Store;

/// Copy store copies the entire state when starting a transaction
pub struct CopyStore {
    stack: Vec<HashMap<String, String>>
}

impl CopyStore {
    pub fn new() -> CopyStore {
        let mut stack = Vec::new();
        stack.push(HashMap::new());
        CopyStore{ stack }
    }
}

impl Store for CopyStore {
    fn read(&self, key: String) -> HandleResult {
        match self.stack.last().unwrap().get(&key) {
            None => HandleResult::Failure(format!("Key not found: {}", key)),
            Some(value) => HandleResult::Result(value.to_owned())
        }
    }

    fn write(&mut self, key: String, value: String) -> HandleResult {
        self.stack.last_mut().unwrap().insert(key, value);
        HandleResult::Success
    }

    fn delete(&mut self, key: String) -> HandleResult {
        self.stack.last_mut().unwrap().remove(&key);
        HandleResult::Success
    }

    fn start(&mut self) -> HandleResult {
        self.stack.push(self.stack.last().unwrap().clone());
        HandleResult::Success
    }

    fn abort(&mut self) -> HandleResult {
        if self.stack.len() == 1 {
            return HandleResult::Failure("no transaction to abort".to_string())
        }
        self.stack.pop();
        HandleResult::Success
    }

    fn commit(&mut self) -> HandleResult {
        if self.stack.len() == 1 {
            return HandleResult::Failure("no transaction to commit".to_string())
        }
        let state = self.stack.pop().unwrap();
        self.stack.pop();
        self.stack.push(state);
        HandleResult::Success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let mut store = CopyStore::new();
        store.write("foo".to_owned(), "bar".to_owned());
        assert_eq!(HandleResult::Result("bar".to_owned()), store.read("foo".to_owned()));
    }

    #[test]
    fn test_commit() {
        let mut store = CopyStore::new();
        store.start();
        store.write("foo".to_owned(), "bar".to_owned());
        assert_eq!(HandleResult::Result("bar".to_owned()), store.read("foo".to_owned()));
        store.commit();
        assert_eq!(HandleResult::Result("bar".to_owned()), store.read("foo".to_owned()));
    }

    #[test]
    fn test_abort() {
        let mut store = CopyStore::new();
        store.start();
        store.write("foo".to_owned(), "bar".to_owned());
        assert_eq!(HandleResult::Result("bar".to_owned()), store.read("foo".to_owned()));
        store.abort();
        assert_eq!(HandleResult::Failure("Key not found: foo".to_owned()), store.read("foo".to_owned()));
    }

    #[test]
    fn test_nested_inner_abort() {
        let mut store = CopyStore::new();
        store.start();
        store.write("foo".to_owned(), "bar".to_owned());
        assert_eq!(HandleResult::Result("bar".to_owned()), store.read("foo".to_owned()));
        store.start();
        store.write("bar".to_owned(), "baz".to_owned());
        store.abort();
        assert_eq!(HandleResult::Result("bar".to_owned()), store.read("foo".to_owned()));
        assert_eq!(HandleResult::Failure("Key not found: bar".to_owned()), store.read("bar".to_owned()));
        store.commit();
        assert_eq!(HandleResult::Result("bar".to_owned()), store.read("foo".to_owned()));
    }

    #[test]
    fn test_nested_outer_abort() {
        let mut store = CopyStore::new();
        store.start();
        store.write("foo".to_owned(), "bar".to_owned());
        assert_eq!(HandleResult::Result("bar".to_owned()), store.read("foo".to_owned()));
        store.start();
        store.write("bar".to_owned(), "baz".to_owned());
        store.commit();
        assert_eq!(HandleResult::Result("bar".to_owned()), store.read("foo".to_owned()));
        assert_eq!(HandleResult::Result("baz".to_owned()), store.read("bar".to_owned()));
        store.abort();
        assert_eq!(HandleResult::Failure("Key not found: foo".to_owned()), store.read("foo".to_owned()));
        assert_eq!(HandleResult::Failure("Key not found: bar".to_owned()), store.read("bar".to_owned()));
    }
}