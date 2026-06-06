use super::ast::*;
use super::error::ParseError;
use super::lexer::Lexer;
use super::token::{Literal, Token};
use crate::types::Value;

pub struct SqlParser {
    tokens: Vec<Token>,
    position: usize,
}

#[allow(clippy::new_without_default)]
impl SqlParser {
    pub fn new() -> Self {
        SqlParser {
            tokens: Vec::new(),
            position: 0,
        }
    }

    pub fn parse(&mut self, sql: &str) -> Result<Statement, ParseError> {
        let mut lexer = Lexer::new(sql);
        self.tokens = lexer.tokenize();
        self.position = 0;

        self.parse_statement()
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.peek() {
            Some(Token::Keyword(k)) if k.to_uppercase() == "SELECT" => {
                Ok(Statement::Select(self.parse_select()?))
            }
            Some(Token::Keyword(k)) if k.to_uppercase() == "INSERT" => {
                Ok(Statement::Insert(self.parse_insert()?))
            }
            Some(Token::Keyword(k)) if k.to_uppercase() == "CREATE" => {
                Ok(Statement::CreateTable(self.parse_create_table()?))
            }
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    fn parse_select(&mut self) -> Result<SelectStatement, ParseError> {
        self.consume_keyword("SELECT")?;

        let mut columns = Vec::new();
        loop {
            columns.push(self.parse_expression()?);
            match self.peek() {
                Some(Token::Comma) => {
                    self.next();
                }
                _ => break,
            }
        }

        let from = if self.match_keyword("FROM") {
            match self.next() {
                Some(Token::Identifier(table)) => Some(table),
                _ => return Err(ParseError::UnexpectedToken),
            }
        } else {
            None
        };

        let where_clause = if self.match_keyword("WHERE") {
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(SelectStatement {
            columns,
            from,
            where_clause,
            limit: None,
            offset: None,
            order_by: vec![],
        })
    }

    fn parse_insert(&mut self) -> Result<InsertStatement, ParseError> {
        self.consume_keyword("INSERT")?;
        self.consume_keyword("INTO")?;

        let table = match self.next() {
            Some(Token::Identifier(t)) => t,
            _ => return Err(ParseError::UnexpectedToken),
        };

        self.expect(&Token::LParen)?;
        let mut columns = Vec::new();
        loop {
            match self.next() {
                Some(Token::Identifier(col)) => columns.push(col),
                _ => return Err(ParseError::UnexpectedToken),
            }
            match self.peek() {
                Some(Token::Comma) => {
                    self.next();
                }
                Some(Token::RParen) => {
                    self.next();
                    break;
                }
                _ => return Err(ParseError::UnexpectedToken),
            }
        }

        self.consume_keyword("VALUES")?;
        let mut all_values = Vec::new();
        loop {
            self.expect(&Token::LParen)?;
            let mut row = Vec::new();
            loop {
                row.push(self.parse_expression()?);
                match self.peek() {
                    Some(Token::Comma) => {
                        self.next();
                    }
                    Some(Token::RParen) => {
                        self.next();
                        break;
                    }
                    _ => return Err(ParseError::UnexpectedToken),
                }
            }
            all_values.push(row);
            match self.peek() {
                Some(Token::Comma) => {
                    self.next();
                }
                _ => break,
            }
        }

        Ok(InsertStatement {
            table,
            columns,
            values: all_values,
        })
    }

    fn parse_create_table(&mut self) -> Result<CreateTableStatement, ParseError> {
        self.consume_keyword("CREATE")?;
        self.consume_keyword("TABLE")?;

        let table = match self.next() {
            Some(Token::Identifier(t)) => t,
            _ => return Err(ParseError::UnexpectedToken),
        };

        self.expect(&Token::LParen)?;
        let mut columns = Vec::new();
        loop {
            let name = match self.next() {
                Some(Token::Identifier(n)) => n,
                _ => return Err(ParseError::UnexpectedToken),
            };
            let data_type = self.parse_data_type()?;
            columns.push(ColumnDef { name, data_type });
            match self.peek() {
                Some(Token::Comma) => {
                    self.next();
                }
                Some(Token::RParen) => {
                    self.next();
                    break;
                }
                _ => return Err(ParseError::UnexpectedToken),
            }
        }

        Ok(CreateTableStatement { table, columns })
    }

    fn parse_data_type(&mut self) -> Result<DataType, ParseError> {
        match self.next() {
            Some(Token::Keyword(k)) => match k.to_uppercase().as_str() {
                "INT" | "INTEGER" => Ok(DataType::Int),
                "FLOAT" | "DOUBLE" => Ok(DataType::Float),
                "BOOLEAN" | "BOOL" => Ok(DataType::Boolean),
                "VARCHAR" | "TEXT" | "STRING" => Ok(DataType::String),
                _ => Err(ParseError::UnexpectedToken),
            },
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        match self.next() {
            Some(Token::Asterisk) => Ok(Expression::Column("*".to_string())),
            Some(Token::Identifier(col)) => Ok(Expression::Column(col)),
            Some(Token::Literal(lit)) => Ok(Expression::Literal(self.convert_literal(lit))),
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    fn convert_literal(&self, lit: Literal) -> Value {
        match lit {
            Literal::Int(i) => Value::Int(i),
            Literal::Float(f) => Value::Float(f),
            Literal::String(s) => Value::String(s),
            Literal::Boolean(b) => Value::Boolean(b),
            Literal::Null => Value::Null,
        }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.position).cloned()
    }

    fn next(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        self.position += 1;
        token
    }

    fn expect(&mut self, expected: &Token) -> Result<(), ParseError> {
        match self.next() {
            Some(t) if &t == expected => Ok(()),
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    fn match_keyword(&mut self, keyword: &str) -> bool {
        match self.peek() {
            Some(Token::Keyword(k)) if k.to_uppercase() == keyword.to_uppercase() => {
                self.position += 1;
                true
            }
            _ => false,
        }
    }

    fn consume_keyword(&mut self, keyword: &str) -> Result<(), ParseError> {
        if self.match_keyword(keyword) {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== SELECT 解析测试 ==========

    #[test]
    fn test_parse_simple_select() {
        let mut parser = SqlParser::new();
        let stmt = parser.parse("SELECT * FROM users").unwrap();
        match stmt {
            Statement::Select(select) => {
                assert_eq!(select.from, Some("users".to_string()));
                assert!(select.where_clause.is_none());
            }
            _ => panic!("Expected SELECT statement"),
        }
    }

    #[test]
    fn test_parse_select_specific_columns() {
        let mut parser = SqlParser::new();
        let stmt = parser.parse("SELECT id, name FROM users").unwrap();
        match stmt {
            Statement::Select(select) => {
                assert_eq!(select.from, Some("users".to_string()));
                assert!(select.where_clause.is_none());
            }
            _ => panic!("Expected SELECT statement"),
        }
    }

    #[test]
    fn test_parse_select_with_where() {
        let mut parser = SqlParser::new();
        let stmt = parser.parse("SELECT * FROM users WHERE id").unwrap();
        match stmt {
            Statement::Select(select) => {
                assert_eq!(select.from, Some("users".to_string()));
                assert!(select.where_clause.is_some());
            }
            _ => panic!("Expected SELECT statement"),
        }
    }

    #[test]
    fn test_parse_select_without_from() {
        let mut parser = SqlParser::new();
        let stmt = parser.parse("SELECT 1").unwrap();
        match stmt {
            Statement::Select(select) => {
                assert!(select.from.is_none());
                assert!(select.where_clause.is_none());
            }
            _ => panic!("Expected SELECT statement"),
        }
    }

    // ========== INSERT 解析测试 ==========

    #[test]
    fn test_parse_insert() {
        let mut parser = SqlParser::new();
        let stmt = parser
            .parse("INSERT INTO users (name, age) VALUES ('Alice', 25)")
            .unwrap();
        match stmt {
            Statement::Insert(insert) => {
                assert_eq!(insert.table, "users");
                assert_eq!(insert.columns, vec!["name", "age"]);
                assert_eq!(insert.values.len(), 1);
                assert_eq!(insert.values[0].len(), 2);
            }
            _ => panic!("Expected INSERT statement"),
        }
    }

    #[test]
    fn test_parse_insert_multi_rows() {
        let mut parser = SqlParser::new();
        let stmt = parser
            .parse("INSERT INTO t (a) VALUES (1), (2), (3)")
            .unwrap();
        match stmt {
            Statement::Insert(insert) => {
                assert_eq!(insert.table, "t");
                assert_eq!(insert.values.len(), 3);
            }
            _ => panic!("Expected INSERT statement"),
        }
    }

    // ========== CREATE TABLE 解析测试 ==========

    #[test]
    fn test_parse_create_table() {
        let mut parser = SqlParser::new();
        let stmt = parser
            .parse("CREATE TABLE users (id INT, name VARCHAR, active BOOLEAN)")
            .unwrap();
        match stmt {
            Statement::CreateTable(create) => {
                assert_eq!(create.table, "users");
                assert_eq!(create.columns.len(), 3);
                assert_eq!(create.columns[0].name, "id");
                assert_eq!(create.columns[1].name, "name");
                assert_eq!(create.columns[2].name, "active");
            }
            _ => panic!("Expected CREATE TABLE statement"),
        }
    }

    #[test]
    fn test_parse_create_table_data_types() {
        let mut parser = SqlParser::new();
        let stmt = parser
            .parse("CREATE TABLE t (a INT, b FLOAT, c BOOLEAN, d VARCHAR)")
            .unwrap();
        match stmt {
            Statement::CreateTable(create) => {
                assert_eq!(create.columns.len(), 4);
            }
            _ => panic!("Expected CREATE TABLE"),
        }
    }

    // ========== 错误处理测试 ==========

    #[test]
    fn test_parse_invalid_sql() {
        let mut parser = SqlParser::new();
        let result = parser.parse("INVALID SQL HERE");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_input() {
        let mut parser = SqlParser::new();
        let result = parser.parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_insert_missing_into() {
        let mut parser = SqlParser::new();
        let result = parser.parse("INSERT users VALUES (1)");
        assert!(result.is_err());
    }

    // ========== SqlParser 构造测试 ==========

    #[test]
    fn test_parser_new() {
        let parser = SqlParser::new();
        assert_eq!(parser.position, 0);
        assert!(parser.tokens.is_empty());
    }
}
