use super::error::ExecError;
use crate::types::RecordBatch;

pub trait Operator {
    fn next(&mut self) -> Result<Option<RecordBatch>, ExecError>;
}

pub struct ScanOperator {
    table_name: String,
    done: bool,
}

impl ScanOperator {
    pub fn new(table_name: String) -> Self {
        ScanOperator {
            table_name,
            done: false,
        }
    }
}

pub struct ProjectOperator {
    #[allow(dead_code)]
    columns: Vec<String>,
    child: Box<dyn Operator>,
}

impl ProjectOperator {
    pub fn new(columns: Vec<String>, child: Box<dyn Operator>) -> Self {
        ProjectOperator { columns, child }
    }
}

pub struct FilterOperator {
    _predicate: String,
    child: Box<dyn Operator>,
}

impl FilterOperator {
    #[allow(dead_code)]
    pub fn new(predicate: String, child: Box<dyn Operator>) -> Self {
        FilterOperator {
            _predicate: predicate,
            child,
        }
    }
}

impl Operator for ScanOperator {
    fn next(&mut self) -> Result<Option<RecordBatch>, ExecError> {
        if self.done {
            Ok(None)
        } else {
            self.done = true;
            Err(ExecError::Unsupported(format!(
                "ScanOperator needs storage: {}",
                self.table_name
            )))
        }
    }
}

impl Operator for ProjectOperator {
    fn next(&mut self) -> Result<Option<RecordBatch>, ExecError> {
        self.child.next()
    }
}

impl Operator for FilterOperator {
    fn next(&mut self) -> Result<Option<RecordBatch>, ExecError> {
        self.child.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_operator_new() {
        let scan = ScanOperator::new("users".to_string());
        assert_eq!(scan.table_name, "users");
        assert!(!scan.done);
    }

    #[test]
    fn test_scan_operator_next_first_call() {
        let mut scan = ScanOperator::new("test_table".to_string());
        let result = scan.next();
        assert!(result.is_err()); // Returns Unsupported error (needs storage)
    }

    #[test]
    fn test_scan_operator_next_second_call() {
        let mut scan = ScanOperator::new("test_table".to_string());
        let _ = scan.next(); // First call sets done=true and errors
        let result = scan.next();
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // done=true returns None
    }

    #[test]
    fn test_project_operator_new() {
        let scan = ScanOperator::new("users".to_string());
        let proj = ProjectOperator::new(vec!["id".to_string(), "name".to_string()], Box::new(scan));
        assert_eq!(proj.columns, vec!["id", "name"]);
    }

    #[test]
    fn test_filter_operator_new() {
        let scan = ScanOperator::new("users".to_string());
        let filter = FilterOperator::new("id > 10".to_string(), Box::new(scan));
        // Just test construction succeeds
        let _ = filter;
    }
}
