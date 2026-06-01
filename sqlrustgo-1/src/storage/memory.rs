use super::engine::{Predicate, StorageEngine};
use super::error::StorageError;
use crate::types::{RecordBatch, Schema, TableSchema, Value};
use std::collections::HashMap;

pub struct MemoryStorage {
    tables: HashMap<String, TableSchema>,
    data: HashMap<String, Vec<Vec<Value>>>,
}

#[allow(clippy::new_without_default)]
impl MemoryStorage {
    pub fn new() -> Self {
        MemoryStorage {
            tables: HashMap::new(),
            data: HashMap::new(),
        }
    }

    fn eval_predicate(row: &[Value], schema: &Schema, predicate: &Predicate) -> bool {
        match predicate {
            Predicate::Eq(col_name, val) => {
                if let Some(idx) = schema.columns().iter().position(|c| c.name() == col_name) {
                    &row[idx] == val
                } else {
                    false
                }
            }
            Predicate::Gt(col_name, val) => {
                if let Some(idx) = schema.columns().iter().position(|c| c.name() == col_name) {
                    matches!((&row[idx], val) , (Value::Int(a), Value::Int(b)) if a > b)
                } else {
                    false
                }
            }
            Predicate::Lt(col_name, val) => {
                if let Some(idx) = schema.columns().iter().position(|c| c.name() == col_name) {
                    matches!((&row[idx], val) , (Value::Int(a), Value::Int(b)) if a < b)
                } else {
                    false
                }
            }
            Predicate::And(a, b) => {
                Self::eval_predicate(row, schema, a) && Self::eval_predicate(row, schema, b)
            }
            Predicate::Or(a, b) => {
                Self::eval_predicate(row, schema, a) || Self::eval_predicate(row, schema, b)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ColumnSchema, DataType, Schema, Value};

    fn make_test_schema() -> Schema {
        Schema::new(vec![
            ColumnSchema::new("id", DataType::Int, false),
            ColumnSchema::new("name", DataType::String, true),
            ColumnSchema::new("age", DataType::Int, true),
        ])
    }

    fn make_test_batch() -> RecordBatch {
        let schema = make_test_schema();
        let mut batch = RecordBatch::new(schema);
        batch.add_row(vec![
            Value::Int(1),
            Value::String("Alice".to_string()),
            Value::Int(25),
        ]);
        batch.add_row(vec![
            Value::Int(2),
            Value::String("Bob".to_string()),
            Value::Int(30),
        ]);
        batch.add_row(vec![
            Value::Int(3),
            Value::String("Charlie".to_string()),
            Value::Int(35),
        ]);
        batch
    }

    // ========== 表创建和删除测试 ==========

    #[test]
    fn test_create_table() {
        let mut storage = MemoryStorage::new();
        let schema = make_test_schema();
        assert!(storage.create_table("users", schema).is_ok());
    }

    #[test]
    fn test_create_table_duplicate() {
        let mut storage = MemoryStorage::new();
        let schema = make_test_schema();
        storage.create_table("users", schema.clone()).unwrap();
        assert!(storage.create_table("users", schema).is_ok()); // Overwrites
    }

    // ========== 读取测试 ==========

    #[test]
    fn test_read_empty_table() {
        let mut storage = MemoryStorage::new();
        storage.create_table("users", make_test_schema()).unwrap();
        let batch = storage.read("users", None).unwrap();
        assert!(batch.is_empty());
        assert_eq!(batch.row_count(), 0);
    }

    #[test]
    fn test_read_nonexistent_table() {
        let storage = MemoryStorage::new();
        let result = storage.read("nonexistent", None);
        assert!(result.is_err());
    }

    // ========== 写入和读取测试 ==========

    #[test]
    fn test_write_and_read() {
        let mut storage = MemoryStorage::new();
        storage.create_table("users", make_test_schema()).unwrap();
        let batch = make_test_batch();
        let count = storage.write("users", batch).unwrap();
        assert_eq!(count, 3);

        let result = storage.read("users", None).unwrap();
        assert_eq!(result.row_count(), 3);
    }

    // ========== 谓词过滤测试 ==========

    #[test]
    fn test_read_with_eq_predicate() {
        let mut storage = MemoryStorage::new();
        storage.create_table("users", make_test_schema()).unwrap();
        storage.write("users", make_test_batch()).unwrap();

        let pred = Predicate::Eq("name".to_string(), Value::String("Alice".to_string()));
        let result = storage.read("users", Some(pred)).unwrap();
        assert_eq!(result.row_count(), 1);
        assert_eq!(result.rows()[0][1], Value::String("Alice".to_string()));
    }

    #[test]
    fn test_read_with_gt_predicate() {
        let mut storage = MemoryStorage::new();
        storage.create_table("users", make_test_schema()).unwrap();
        storage.write("users", make_test_batch()).unwrap();

        let pred = Predicate::Gt("age".to_string(), Value::Int(25));
        let result = storage.read("users", Some(pred)).unwrap();
        assert_eq!(result.row_count(), 2); // Bob(30) and Charlie(35)
    }

    #[test]
    fn test_read_with_lt_predicate() {
        let mut storage = MemoryStorage::new();
        storage.create_table("users", make_test_schema()).unwrap();
        storage.write("users", make_test_batch()).unwrap();

        let pred = Predicate::Lt("age".to_string(), Value::Int(30));
        let result = storage.read("users", Some(pred)).unwrap();
        assert_eq!(result.row_count(), 1); // Alice(25)
    }

    #[test]
    fn test_read_with_and_predicate() {
        let mut storage = MemoryStorage::new();
        storage.create_table("users", make_test_schema()).unwrap();
        storage.write("users", make_test_batch()).unwrap();

        let pred = Predicate::And(
            Box::new(Predicate::Gt("age".to_string(), Value::Int(20))),
            Box::new(Predicate::Lt("age".to_string(), Value::Int(35))),
        );
        let result = storage.read("users", Some(pred)).unwrap();
        assert_eq!(result.row_count(), 2); // Alice(25), Bob(30)
    }

    #[test]
    fn test_read_with_or_predicate() {
        let mut storage = MemoryStorage::new();
        storage.create_table("users", make_test_schema()).unwrap();
        storage.write("users", make_test_batch()).unwrap();

        let pred = Predicate::Or(
            Box::new(Predicate::Eq(
                "name".to_string(),
                Value::String("Alice".to_string()),
            )),
            Box::new(Predicate::Eq(
                "name".to_string(),
                Value::String("Charlie".to_string()),
            )),
        );
        let result = storage.read("users", Some(pred)).unwrap();
        assert_eq!(result.row_count(), 2);
    }

    // ========== 删除测试 ==========

    #[test]
    fn test_delete_with_predicate() {
        let mut storage = MemoryStorage::new();
        storage.create_table("users", make_test_schema()).unwrap();
        storage.write("users", make_test_batch()).unwrap();

        let pred = Predicate::Eq("name".to_string(), Value::String("Alice".to_string()));
        let deleted = storage.delete("users", pred).unwrap();
        assert_eq!(deleted, 1);

        let result = storage.read("users", None).unwrap();
        assert_eq!(result.row_count(), 2);
    }

    #[test]
    fn test_delete_nonexistent_table() {
        let mut storage = MemoryStorage::new();
        let pred = Predicate::Eq("x".to_string(), Value::Int(1));
        let result = storage.delete("nonexistent", pred);
        assert!(result.is_err());
    }

    // ========== 表名枚举测试 ==========

    #[test]
    fn test_multiple_tables() {
        let mut storage = MemoryStorage::new();
        let schema1 = Schema::new(vec![ColumnSchema::new("a", DataType::Int, false)]);
        let schema2 = Schema::new(vec![ColumnSchema::new("b", DataType::String, false)]);

        storage.create_table("t1", schema1.clone()).unwrap();
        storage.create_table("t2", schema2.clone()).unwrap();

        let mut batch1 = RecordBatch::new(schema1);
        batch1.add_row(vec![Value::Int(100)]);

        let mut batch2 = RecordBatch::new(schema2);
        batch2.add_row(vec![Value::String("hello".to_string())]);

        storage.write("t1", batch1).unwrap();
        storage.write("t2", batch2).unwrap();

        assert_eq!(storage.read("t1", None).unwrap().row_count(), 1);
        assert_eq!(storage.read("t2", None).unwrap().row_count(), 1);
    }

    // ========== MemoryStorage 构造测试 ==========

    #[test]
    fn test_memory_storage_new() {
        let storage = MemoryStorage::new();
        assert!(storage.tables.is_empty());
        assert!(storage.data.is_empty());
    }
}

impl StorageEngine for MemoryStorage {
    fn read(&self, table: &str, predicate: Option<Predicate>) -> Result<RecordBatch, StorageError> {
        let table_schema = self.tables.get(table).ok_or(StorageError::TableNotFound)?;
        let rows = self.data.get(table).ok_or(StorageError::TableNotFound)?;

        let schema = table_schema.schema().clone();
        let mut batch = RecordBatch::new(schema.clone());

        for row in rows {
            if let Some(ref pred) = predicate {
                if Self::eval_predicate(row, &schema, pred) {
                    batch.add_row(row.clone());
                }
            } else {
                batch.add_row(row.clone());
            }
        }

        Ok(batch)
    }

    fn write(&mut self, table: &str, batch: RecordBatch) -> Result<usize, StorageError> {
        let rows = self
            .data
            .get_mut(table)
            .ok_or(StorageError::TableNotFound)?;
        let count = batch.row_count();
        for row in batch.rows() {
            rows.push(row.clone());
        }
        Ok(count)
    }

    fn delete(&mut self, table: &str, predicate: Predicate) -> Result<usize, StorageError> {
        let table_schema = self.tables.get(table).ok_or(StorageError::TableNotFound)?;
        let schema = table_schema.schema();
        let original_len = self
            .data
            .get(table)
            .ok_or(StorageError::TableNotFound)?
            .len();

        self.data
            .get_mut(table)
            .ok_or(StorageError::TableNotFound)?
            .retain(|row| !Self::eval_predicate(row, schema, &predicate));

        let new_len = self
            .data
            .get(table)
            .ok_or(StorageError::TableNotFound)?
            .len();
        Ok(original_len - new_len)
    }

    fn create_table(&mut self, name: &str, schema: Schema) -> Result<(), StorageError> {
        let table_schema = TableSchema::new(name, schema);
        self.tables.insert(name.to_string(), table_schema);
        self.data.insert(name.to_string(), Vec::<Vec<Value>>::new());
        Ok(())
    }
}
