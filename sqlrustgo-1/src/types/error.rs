use std::fmt;

#[derive(Debug)]
pub enum TypeError {
    TypeMismatch,
    InvalidCast,
    ColumnNotFound(String),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::TypeMismatch => write!(f, "Type mismatch"),
            TypeError::InvalidCast => write!(f, "Invalid type cast"),
            TypeError::ColumnNotFound(name) => write!(f, "Column not found: {}", name),
        }
    }
}

impl std::error::Error for TypeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_error_display() {
        assert_eq!(format!("{}", TypeError::TypeMismatch), "Type mismatch");
        assert_eq!(format!("{}", TypeError::InvalidCast), "Invalid type cast");
        assert_eq!(
            format!("{}", TypeError::ColumnNotFound("age".to_string())),
            "Column not found: age"
        );
    }
}
