use super::error::ExecError;
use crate::parser::{ast, Statement};
use crate::storage::StorageEngine;
use crate::types::DataType as SchemaDataType;
use crate::types::{ColumnSchema, RecordBatch, Schema};
use std::sync::Arc;

pub struct Executor<E: StorageEngine> {
    storage: Arc<E>,
}

impl<E: StorageEngine> Executor<E> {
    pub fn new(storage: Arc<E>) -> Self {
        Executor { storage }
    }

    pub fn execute(&mut self, stmt: Statement) -> Result<Vec<RecordBatch>, ExecError> {
        match stmt {
            Statement::Select(select) => self.execute_select(select),
            Statement::Insert(insert) => self.execute_insert(insert),
            Statement::CreateTable(create) => self.execute_create_table(create),
        }
    }

    fn execute_select(
        &mut self,
        select: ast::SelectStatement,
    ) -> Result<Vec<RecordBatch>, ExecError> {
        let table_name = select.from.ok_or(ExecError::MissingTable)?;

        let predicate = None;
        let batch = Arc::get_mut(&mut self.storage)
            .unwrap()
            .read(&table_name, predicate)?;

        Ok(vec![batch])
    }

    fn execute_insert(
        &mut self,
        insert: ast::InsertStatement,
    ) -> Result<Vec<RecordBatch>, ExecError> {
        let batch = Arc::get_mut(&mut self.storage)
            .unwrap()
            .read(&insert.table, None)?;

        let schema = batch.schema().clone();
        let mut new_batch = RecordBatch::new(schema);

        for row_values in insert.values {
            let mut row = Vec::new();
            for expr in row_values {
                match expr {
                    ast::Expression::Literal(v) => row.push(v),
                    _ => return Err(ExecError::UnsupportedExpression),
                }
            }
            new_batch.add_row(row);
        }

        Arc::get_mut(&mut self.storage)
            .unwrap()
            .write(&insert.table, new_batch)?;

        Ok(vec![])
    }

    fn execute_create_table(
        &mut self,
        create: ast::CreateTableStatement,
    ) -> Result<Vec<RecordBatch>, ExecError> {
        let mut columns = Vec::new();
        for col in create.columns {
            let data_type = match col.data_type {
                ast::DataType::Int => SchemaDataType::Int,
                ast::DataType::Float => SchemaDataType::Float,
                ast::DataType::Boolean => SchemaDataType::Boolean,
                ast::DataType::String => SchemaDataType::String,
            };
            columns.push(ColumnSchema::new(&col.name, data_type, true));
        }

        let schema = Schema::new(columns);
        Arc::get_mut(&mut self.storage)
            .unwrap()
            .create_table(&create.table, schema)?;

        Ok(vec![])
    }
}
