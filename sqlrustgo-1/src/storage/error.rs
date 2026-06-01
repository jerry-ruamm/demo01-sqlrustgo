use std::fmt;

#[derive(Debug)]
pub enum StorageError {
    TableNotFound,
    ColumnNotFound,
    TypeMismatch,
    IoError,
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::TableNotFound => write!(f, "Table not found"),
            StorageError::ColumnNotFound => write!(f, "Column not found"),
            StorageError::TypeMismatch => write!(f, "Type mismatch"),
            StorageError::IoError => write!(f, "IO error"),
        }
    }
}

impl std::error::Error for StorageError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_error_display() {
        assert_eq!(
            format!("{}", StorageError::TableNotFound),
            "Table not found"
        );
        assert_eq!(
            format!("{}", StorageError::ColumnNotFound),
            "Column not found"
        );
        assert_eq!(format!("{}", StorageError::TypeMismatch), "Type mismatch");
        assert_eq!(format!("{}", StorageError::IoError), "IO error");
    }

    #[test]
    fn test_storage_error_debug() {
        let err = StorageError::TableNotFound;
        assert_eq!(format!("{:?}", err), "TableNotFound");
    }

    #[test]
    fn test_storage_error_is_error_trait() {
        fn assert_error<T: std::error::Error>() {}
        assert_error::<StorageError>();
    }
}
