use crate::types::TableSchema;
use std::collections::HashMap;

pub struct Catalog {
    tables: HashMap<String, TableSchema>,
}

#[allow(clippy::new_without_default)]
impl Catalog {
    pub fn new() -> Self {
        Catalog {
            tables: HashMap::new(),
        }
    }

    pub fn register_table(&mut self, table: TableSchema) {
        self.tables.insert(table.name().to_string(), table);
    }

    pub fn get_table(&self, name: &str) -> Option<&TableSchema> {
        self.tables.get(name)
    }

    pub fn list_tables(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ColumnSchema, DataType, Schema};

    fn make_table_schema(name: &str) -> TableSchema {
        let columns = vec![
            ColumnSchema::new("id", DataType::Int, false),
            ColumnSchema::new("name", DataType::String, true),
        ];
        TableSchema::new(name, Schema::new(columns))
    }

    #[test]
    fn test_catalog_new() {
        let catalog = Catalog::new();
        assert!(catalog.list_tables().is_empty());
    }

    #[test]
    fn test_register_and_get_table() {
        let mut catalog = Catalog::new();
        let ts = make_table_schema("users");
        catalog.register_table(ts);

        let found = catalog.get_table("users");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name(), "users");
    }

    #[test]
    fn test_get_nonexistent_table() {
        let catalog = Catalog::new();
        assert!(catalog.get_table("nonexistent").is_none());
    }

    #[test]
    fn test_list_tables() {
        let mut catalog = Catalog::new();
        catalog.register_table(make_table_schema("users"));
        catalog.register_table(make_table_schema("orders"));
        catalog.register_table(make_table_schema("products"));

        let tables = catalog.list_tables();
        assert_eq!(tables.len(), 3);
        assert!(tables.contains(&"users".to_string()));
        assert!(tables.contains(&"orders".to_string()));
        assert!(tables.contains(&"products".to_string()));
    }

    #[test]
    fn test_register_overwrite() {
        let mut catalog = Catalog::new();
        catalog.register_table(make_table_schema("users"));
        catalog.register_table(make_table_schema("users"));

        assert_eq!(catalog.list_tables().len(), 1);
    }
}
