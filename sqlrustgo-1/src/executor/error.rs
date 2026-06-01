use crate::storage::StorageError;
use std::fmt;

#[derive(Debug)]
pub enum ExecError {
    StorageError(StorageError),
    MissingTable,
    UnsupportedExpression,
    Unsupported(String),
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecError::StorageError(e) => write!(f, "Storage error: {}", e),
            ExecError::MissingTable => write!(f, "Missing table in SELECT statement"),
            ExecError::UnsupportedExpression => write!(f, "Unsupported expression"),
            ExecError::Unsupported(msg) => write!(f, "Unsupported: {}", msg),
        }
    }
}

impl std::error::Error for ExecError {}

impl From<StorageError> for ExecError {
    fn from(err: StorageError) -> Self {
        ExecError::StorageError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_error_display() {
        assert_eq!(
            format!("{}", ExecError::MissingTable),
            "Missing table in SELECT statement"
        );
        assert_eq!(
            format!("{}", ExecError::UnsupportedExpression),
            "Unsupported expression"
        );
        assert_eq!(
            format!("{}", ExecError::Unsupported("test".to_string())),
            "Unsupported: test"
        );
    }

    #[test]
    fn test_exec_error_from_storage_error() {
        let storage_err = StorageError::TableNotFound;
        let exec_err: ExecError = storage_err.into();
        assert_eq!(format!("{}", exec_err), "Storage error: Table not found");
    }
}
