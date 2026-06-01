use super::schema::Schema;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "NULL"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "'{}'", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecordBatch {
    schema: Schema,
    rows: Vec<Vec<Value>>,
}

impl RecordBatch {
    pub fn new(schema: Schema) -> Self {
        RecordBatch {
            schema,
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<Value>) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[Vec<Value>] {
        &self.rows
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::schema::{ColumnSchema, DataType, Schema};

    // ========== Value Display 测试 ==========

    #[test]
    fn test_value_display_null() {
        assert_eq!(format!("{}", Value::Null), "NULL");
    }

    #[test]
    fn test_value_display_boolean() {
        assert_eq!(format!("{}", Value::Boolean(true)), "true");
        assert_eq!(format!("{}", Value::Boolean(false)), "false");
    }

    #[test]
    fn test_value_display_int() {
        assert_eq!(format!("{}", Value::Int(42)), "42");
        assert_eq!(format!("{}", Value::Int(-10)), "-10");
    }

    #[test]
    fn test_value_display_float() {
        assert_eq!(format!("{}", Value::Float(3.14)), "3.14");
    }

    #[test]
    fn test_value_display_string() {
        assert_eq!(format!("{}", Value::String("hello".to_string())), "'hello'");
    }

    #[test]
    fn test_value_clone_eq() {
        let v1 = Value::Int(100);
        let v2 = v1.clone();
        assert_eq!(v1, v2);
    }

    // ========== RecordBatch 测试 ==========

    #[test]
    fn test_record_batch_new() {
        let schema = Schema::new(vec![ColumnSchema::new("id", DataType::Int, false)]);
        let batch = RecordBatch::new(schema.clone());
        assert!(batch.is_empty());
        assert_eq!(batch.row_count(), 0);
        assert_eq!(batch.schema().column_count(), 1);
    }

    #[test]
    fn test_record_batch_add_row() {
        let schema = Schema::new(vec![
            ColumnSchema::new("id", DataType::Int, false),
            ColumnSchema::new("name", DataType::String, true),
        ]);
        let mut batch = RecordBatch::new(schema);
        batch.add_row(vec![Value::Int(1), Value::String("Alice".to_string())]);
        batch.add_row(vec![Value::Int(2), Value::String("Bob".to_string())]);

        assert_eq!(batch.row_count(), 2);
        assert!(!batch.is_empty());
        assert_eq!(batch.rows().len(), 2);
        assert_eq!(batch.rows()[0][0], Value::Int(1));
        assert_eq!(batch.rows()[1][1], Value::String("Bob".to_string()));
    }

    #[test]
    fn test_record_batch_rows_is_slice() {
        let schema = Schema::new(vec![ColumnSchema::new("x", DataType::Int, false)]);
        let mut batch = RecordBatch::new(schema);
        batch.add_row(vec![Value::Int(99)]);
        let rows: &[Vec<Value>] = batch.rows();
        assert_eq!(rows.len(), 1);
    }
}
