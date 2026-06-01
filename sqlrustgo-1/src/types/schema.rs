#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Null,
    Boolean,
    Int,
    Float,
    String,
}

#[derive(Debug, Clone)]
pub struct ColumnSchema {
    name: String,
    data_type: DataType,
    nullable: bool,
}

impl ColumnSchema {
    pub fn new(name: &str, data_type: DataType, nullable: bool) -> Self {
        ColumnSchema {
            name: name.to_string(),
            data_type,
            nullable,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable
    }
}

#[derive(Debug, Clone)]
pub struct Schema {
    columns: Vec<ColumnSchema>,
}

impl Schema {
    pub fn new(columns: Vec<ColumnSchema>) -> Self {
        Schema { columns }
    }

    pub fn columns(&self) -> &[ColumnSchema] {
        &self.columns
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn get_column(&self, index: usize) -> Option<&ColumnSchema> {
        self.columns.get(index)
    }

    pub fn get_column_by_name(&self, name: &str) -> Option<&ColumnSchema> {
        self.columns.iter().find(|c| c.name == name)
    }
}

#[derive(Debug, Clone)]
pub struct TableSchema {
    name: String,
    schema: Schema,
}

impl TableSchema {
    pub fn new(name: &str, schema: Schema) -> Self {
        TableSchema {
            name: name.to_string(),
            schema,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== ColumnSchema 测试 ==========

    #[test]
    fn test_column_schema_creation() {
        let col = ColumnSchema::new("id", DataType::Int, false);
        assert_eq!(col.name(), "id");
        assert_eq!(col.data_type(), &DataType::Int);
        assert!(!col.is_nullable());
    }

    #[test]
    fn test_column_schema_nullable() {
        let col = ColumnSchema::new("email", DataType::String, true);
        assert_eq!(col.name(), "email");
        assert_eq!(col.data_type(), &DataType::String);
        assert!(col.is_nullable());
    }

    // ========== Schema 测试 ==========

    #[test]
    fn test_schema_new() {
        let cols = vec![
            ColumnSchema::new("a", DataType::Int, false),
            ColumnSchema::new("b", DataType::Boolean, true),
        ];
        let schema = Schema::new(cols);
        assert_eq!(schema.column_count(), 2);
    }

    #[test]
    fn test_schema_empty() {
        let schema = Schema::new(vec![]);
        assert_eq!(schema.column_count(), 0);
        assert!(schema.columns().is_empty());
    }

    #[test]
    fn test_schema_get_column() {
        let cols = vec![
            ColumnSchema::new("a", DataType::Int, false),
            ColumnSchema::new("b", DataType::String, true),
        ];
        let schema = Schema::new(cols);
        assert!(schema.get_column(0).is_some());
        assert_eq!(schema.get_column(0).unwrap().name(), "a");
        assert!(schema.get_column(1).is_some());
        assert!(schema.get_column(2).is_none());
    }

    #[test]
    fn test_schema_get_column_by_name() {
        let cols = vec![
            ColumnSchema::new("id", DataType::Int, false),
            ColumnSchema::new("name", DataType::String, true),
        ];
        let schema = Schema::new(cols);
        let found = schema.get_column_by_name("name");
        assert!(found.is_some());
        assert_eq!(found.unwrap().data_type(), &DataType::String);

        assert!(schema.get_column_by_name("nonexistent").is_none());
    }

    // ========== TableSchema 测试 ==========

    #[test]
    fn test_table_schema_creation() {
        let cols = vec![ColumnSchema::new("id", DataType::Int, false)];
        let schema = Schema::new(cols);
        let table = TableSchema::new("users", schema);
        assert_eq!(table.name(), "users");
        assert_eq!(table.schema().column_count(), 1);
    }

    // ========== DataType 测试 ==========

    #[test]
    fn test_data_type_eq() {
        assert_eq!(DataType::Null, DataType::Null);
        assert_eq!(DataType::Int, DataType::Int);
        assert_ne!(DataType::Int, DataType::String);
    }
}
