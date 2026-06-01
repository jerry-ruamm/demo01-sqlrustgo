use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    UnexpectedEof,
    InvalidSyntax,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken => write!(f, "Unexpected token"),
            ParseError::UnexpectedEof => write!(f, "Unexpected end of input"),
            ParseError::InvalidSyntax => write!(f, "Invalid syntax"),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_display() {
        assert_eq!(
            format!("{}", ParseError::UnexpectedToken),
            "Unexpected token"
        );
        assert_eq!(
            format!("{}", ParseError::UnexpectedEof),
            "Unexpected end of input"
        );
        assert_eq!(format!("{}", ParseError::InvalidSyntax), "Invalid syntax");
    }

    #[test]
    fn test_parse_error_debug() {
        assert_eq!(
            format!("{:?}", ParseError::UnexpectedToken),
            "UnexpectedToken"
        );
    }

    #[test]
    fn test_parse_error_is_error_trait() {
        fn assert_error<T: std::error::Error>() {}
        assert_error::<ParseError>();
    }
}
