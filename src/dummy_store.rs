use crate::enums::HandleResult;
use crate::store::Store;

/// Dummy store does not store any values
/// It just echoes back the user input
pub struct DummyStore {

}

impl Store for DummyStore {
    fn read(&self, key: String) -> HandleResult {
        HandleResult::Result(format!("READ: `{}`", key))
    }

    fn write(&mut self, key: String, value: String) -> HandleResult {
        HandleResult::Result(format!("WRITE: `{}` `{}`", key, value))
    }

    fn delete(&mut self, key: String) -> HandleResult {
        HandleResult::Result(format!("DELETE: `{}`", key))
    }

    fn start(&mut self) -> HandleResult {
        HandleResult::Result("START".to_string())
    }

    fn abort(&mut self) -> HandleResult {
        HandleResult::Result("ABORT".to_string())
    }

    fn commit(&mut self) -> HandleResult {
        HandleResult::Result("COMMIT".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let store = DummyStore{};
        assert_eq!(HandleResult::Result("READ: `foo`".to_owned()), store.read("foo".to_owned()));
    }
}