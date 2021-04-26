use crate::enums::HandleResult;

pub trait Store {
    fn read(&self, key: String) -> HandleResult;
    fn write(&mut self, key: String, value: String) -> HandleResult;
    fn delete(&mut self, key: String) -> HandleResult;
    fn start(&mut self) -> HandleResult;
    fn abort(&mut self) -> HandleResult;
    fn commit(&mut self) -> HandleResult;
}