#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Literal(Literal),
    Operator(String),
    LParen,
    RParen,
    Comma,
    Semicolon,
    Asterisk,
    Dot,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(k) => write!(f, "KEYWORD({})", k),
            Token::Identifier(i) => write!(f, "IDENT({})", i),
            Token::Literal(l) => write!(f, "LITERAL({:?})", l),
            Token::Operator(o) => write!(f, "OP({})", o),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Asterisk => write!(f, "*"),
            Token::Dot => write!(f, "."),
            Token::Eof => write!(f, "EOF"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_display_keyword() {
        let token = Token::Keyword("SELECT".to_string());
        assert_eq!(format!("{}", token), "KEYWORD(SELECT)");
    }

    #[test]
    fn test_token_display_identifier() {
        let token = Token::Identifier("my_col".to_string());
        assert_eq!(format!("{}", token), "IDENT(my_col)");
    }

    #[test]
    fn test_token_display_literal_int() {
        let token = Token::Literal(Literal::Int(42));
        assert_eq!(format!("{}", token), "LITERAL(Int(42))");
    }

    #[test]
    fn test_token_display_literal_float() {
        let token = Token::Literal(Literal::Float(3.14));
        assert_eq!(format!("{}", token), "LITERAL(Float(3.14))");
    }

    #[test]
    fn test_token_display_literal_string() {
        let token = Token::Literal(Literal::String("hello".to_string()));
        assert_eq!(format!("{}", token), "LITERAL(String(\"hello\"))");
    }

    #[test]
    fn test_token_display_literal_boolean() {
        let token = Token::Literal(Literal::Boolean(true));
        assert_eq!(format!("{}", token), "LITERAL(Boolean(true))");
    }

    #[test]
    fn test_token_display_literal_null() {
        let token = Token::Literal(Literal::Null);
        assert_eq!(format!("{}", token), "LITERAL(Null)");
    }

    #[test]
    fn test_token_display_operator() {
        let token = Token::Operator("=".to_string());
        assert_eq!(format!("{}", token), "OP(=)");
    }

    #[test]
    fn test_token_display_punctuation() {
        assert_eq!(format!("{}", Token::LParen), "(");
        assert_eq!(format!("{}", Token::RParen), ")");
        assert_eq!(format!("{}", Token::Comma), ",");
        assert_eq!(format!("{}", Token::Semicolon), ";");
        assert_eq!(format!("{}", Token::Asterisk), "*");
        assert_eq!(format!("{}", Token::Dot), ".");
        assert_eq!(format!("{}", Token::Eof), "EOF");
    }

    #[test]
    fn test_token_debug_clone_partial_eq() {
        let t1 = Token::Keyword("SELECT".to_string());
        let t2 = t1.clone();
        assert_eq!(t1, t2);
        assert_eq!(format!("{:?}", t1), "Keyword(\"SELECT\")");
    }
}
