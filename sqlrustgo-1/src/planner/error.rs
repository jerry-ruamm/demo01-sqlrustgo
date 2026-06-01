use std::fmt;

#[derive(Debug)]
pub enum PlanError {
    MissingTable,
    InvalidExpression,
    Unsupported,
}

impl fmt::Display for PlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanError::MissingTable => write!(f, "Missing table name"),
            PlanError::InvalidExpression => write!(f, "Invalid expression"),
            PlanError::Unsupported => write!(f, "Unsupported plan"),
        }
    }
}

impl std::error::Error for PlanError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_error_display() {
        assert_eq!(format!("{}", PlanError::MissingTable), "Missing table name");
        assert_eq!(
            format!("{}", PlanError::InvalidExpression),
            "Invalid expression"
        );
        assert_eq!(format!("{}", PlanError::Unsupported), "Unsupported plan");
    }

    #[test]
    fn test_plan_error_debug() {
        assert_eq!(format!("{:?}", PlanError::Unsupported), "Unsupported");
    }
}
