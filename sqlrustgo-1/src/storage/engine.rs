use super::error::StorageError;
use crate::types::{RecordBatch, Value};

pub enum Predicate {
    Eq(String, Value),
    Gt(String, Value),
    Lt(String, Value),
    And(Box<Predicate>, Box<Predicate>),
    Or(Box<Predicate>, Box<Predicate>),
}

pub trait StorageEngine {
    fn read(&self, table: &str, predicate: Option<Predicate>) -> Result<RecordBatch, StorageError>;
    fn write(&mut self, table: &str, batch: RecordBatch) -> Result<usize, StorageError>;
    fn delete(&mut self, table: &str, predicate: Predicate) -> Result<usize, StorageError>;
    fn create_table(
        &mut self,
        name: &str,
        schema: crate::types::Schema,
    ) -> Result<(), StorageError>;
}
