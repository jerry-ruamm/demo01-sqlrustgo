use super::token::{Literal, Token};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    #[allow(dead_code)]
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            if token == Token::Eof {
                break;
            }
            tokens.push(token);
        }
        tokens.push(Token::Eof);
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let ch = match self.input.peek() {
            Some(c) => *c,
            None => return Some(Token::Eof),
        };

        match ch {
            'a'..='z' | 'A'..='Z' | '_' => Some(self.read_identifier()),
            '0'..='9' => Some(self.read_number()),
            '\'' | '"' => Some(self.read_string()),
            '(' => {
                self.input.next();
                Some(Token::LParen)
            }
            ')' => {
                self.input.next();
                Some(Token::RParen)
            }
            ',' => {
                self.input.next();
                Some(Token::Comma)
            }
            ';' => {
                self.input.next();
                Some(Token::Semicolon)
            }
            '*' => {
                self.input.next();
                Some(Token::Asterisk)
            }
            '.' => {
                self.input.next();
                Some(Token::Dot)
            }
            '=' | '<' | '>' | '!' | '+' | '-' | '/' => Some(self.read_operator()),
            _ => {
                self.input.next();
                Some(Token::Eof)
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.input.next();
            } else {
                break;
            }
        }

        let upper = ident.to_uppercase();
        match upper.as_str() {
            "SELECT" | "FROM" | "WHERE" | "INSERT" | "INTO" | "VALUES" | "UPDATE" | "SET"
            | "DELETE" | "CREATE" | "TABLE" | "AND" | "OR" | "NOT" | "NULL" | "TRUE" | "FALSE"
            | "INT" | "VARCHAR" | "TEXT" | "BOOLEAN" | "FLOAT" | "PRIMARY" | "KEY" | "LIMIT"
            | "OFFSET" | "ORDER" | "BY" | "ASC" | "DESC" => Token::Keyword(ident),
            _ => Token::Identifier(ident),
        }
    }

    fn read_number(&mut self) -> Token {
        let mut num = String::new();
        let mut is_float = false;

        while let Some(&ch) = self.input.peek() {
            match ch {
                '0'..='9' => {
                    num.push(ch);
                    self.input.next();
                }
                '.' if !is_float => {
                    is_float = true;
                    num.push(ch);
                    self.input.next();
                }
                _ => break,
            }
        }

        if is_float {
            if let Ok(f) = num.parse::<f64>() {
                return Token::Literal(Literal::Float(f));
            }
        } else if let Ok(i) = num.parse::<i64>() {
            return Token::Literal(Literal::Int(i));
        }

        Token::Literal(Literal::Int(0))
    }

    fn read_string(&mut self) -> Token {
        let quote = self.input.next().unwrap();
        let mut s = String::new();

        while let Some(&ch) = self.input.peek() {
            if ch == quote {
                self.input.next();
                break;
            } else {
                s.push(ch);
                self.input.next();
            }
        }

        Token::Literal(Literal::String(s))
    }

    fn read_operator(&mut self) -> Token {
        let mut op = String::new();
        op.push(self.input.next().unwrap());

        if let Some(&ch) = self.input.peek() {
            if ch == '=' {
                op.push(ch);
                self.input.next();
            }
        }

        Token::Operator(op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::token::{Literal, Token};

    // ========== 关键字识别测试 ==========

    #[test]
    fn test_keyword_select() {
        let mut lexer = Lexer::new("SELECT");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("SELECT".to_string()));
    }

    #[test]
    fn test_keyword_from() {
        let mut lexer = Lexer::new("FROM");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("FROM".to_string()));
    }

    #[test]
    fn test_keyword_where() {
        let mut lexer = Lexer::new("WHERE");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("WHERE".to_string()));
    }

    #[test]
    fn test_keyword_insert_into_values() {
        let mut lexer = Lexer::new("INSERT INTO VALUES");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("INSERT".to_string()));
        assert_eq!(tokens[1], Token::Keyword("INTO".to_string()));
        assert_eq!(tokens[2], Token::Keyword("VALUES".to_string()));
    }

    #[test]
    fn test_keyword_update_set_delete() {
        let mut lexer = Lexer::new("UPDATE SET DELETE");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("UPDATE".to_string()));
        assert_eq!(tokens[1], Token::Keyword("SET".to_string()));
        assert_eq!(tokens[2], Token::Keyword("DELETE".to_string()));
    }

    #[test]
    fn test_keyword_create_table() {
        let mut lexer = Lexer::new("CREATE TABLE");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("CREATE".to_string()));
        assert_eq!(tokens[1], Token::Keyword("TABLE".to_string()));
    }

    #[test]
    fn test_keyword_and_or_not() {
        let mut lexer = Lexer::new("AND OR NOT");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("AND".to_string()));
        assert_eq!(tokens[1], Token::Keyword("OR".to_string()));
        assert_eq!(tokens[2], Token::Keyword("NOT".to_string()));
    }

    #[test]
    fn test_keyword_null_true_false() {
        let mut lexer = Lexer::new("NULL TRUE FALSE");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("NULL".to_string()));
        assert_eq!(tokens[1], Token::Keyword("TRUE".to_string()));
        assert_eq!(tokens[2], Token::Keyword("FALSE".to_string()));
    }

    #[test]
    fn test_keyword_data_types() {
        let mut lexer = Lexer::new("INT VARCHAR TEXT BOOLEAN FLOAT");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("INT".to_string()));
        assert_eq!(tokens[1], Token::Keyword("VARCHAR".to_string()));
        assert_eq!(tokens[2], Token::Keyword("TEXT".to_string()));
        assert_eq!(tokens[3], Token::Keyword("BOOLEAN".to_string()));
        assert_eq!(tokens[4], Token::Keyword("FLOAT".to_string()));
    }

    #[test]
    fn test_keyword_limit_offset() {
        let mut lexer = Lexer::new("LIMIT OFFSET");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("LIMIT".to_string()));
        assert_eq!(tokens[1], Token::Keyword("OFFSET".to_string()));
    }

    #[test]
    fn test_keyword_case_insensitive() {
        let mut lexer = Lexer::new("select Select SELECT");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("select".to_string()));
        assert_eq!(tokens[1], Token::Keyword("Select".to_string()));
        assert_eq!(tokens[2], Token::Keyword("SELECT".to_string()));
    }

    // ========== 标识符识别测试 ==========

    #[test]
    fn test_identifier_simple() {
        let mut lexer = Lexer::new("my_table column1");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Identifier("my_table".to_string()));
        assert_eq!(tokens[1], Token::Identifier("column1".to_string()));
    }

    #[test]
    fn test_identifier_with_underscore() {
        let mut lexer = Lexer::new("_private_var __dunder__");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Identifier("_private_var".to_string()));
        assert_eq!(tokens[1], Token::Identifier("__dunder__".to_string()));
    }

    #[test]
    fn test_identifier_mixed_case() {
        let mut lexer = Lexer::new("MyTable UserName");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Identifier("MyTable".to_string()));
        assert_eq!(tokens[1], Token::Identifier("UserName".to_string()));
    }

    // ========== 数字字面量测试 ==========

    #[test]
    fn test_integer_literal() {
        let mut lexer = Lexer::new("42 0 999");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Literal(Literal::Int(42)));
        assert_eq!(tokens[1], Token::Literal(Literal::Int(0)));
        assert_eq!(tokens[2], Token::Literal(Literal::Int(999)));
    }

    #[test]
    fn test_float_literal() {
        let mut lexer = Lexer::new("3.14 0.5 10.0");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Literal(Literal::Float(3.14)));
        assert_eq!(tokens[1], Token::Literal(Literal::Float(0.5)));
        assert_eq!(tokens[2], Token::Literal(Literal::Float(10.0)));
    }

    // ========== 字符串字面量测试 ==========

    #[test]
    fn test_string_literal_single_quote() {
        let mut lexer = Lexer::new("'hello world'");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens[0],
            Token::Literal(Literal::String("hello world".to_string()))
        );
    }

    #[test]
    fn test_string_literal_double_quote() {
        let mut lexer = Lexer::new("\"hello world\"");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens[0],
            Token::Literal(Literal::String("hello world".to_string()))
        );
    }

    #[test]
    fn test_empty_string() {
        let mut lexer = Lexer::new("''");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Literal(Literal::String("".to_string())));
    }

    // ========== 运算符识别测试 ==========

    #[test]
    fn test_arithmetic_operators() {
        let mut lexer = Lexer::new("+ - * /");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Operator("+".to_string()));
        assert_eq!(tokens[1], Token::Operator("-".to_string()));
        assert_eq!(tokens[2], Token::Asterisk);
        assert_eq!(tokens[3], Token::Operator("/".to_string()));
    }

    #[test]
    fn test_comparison_operators() {
        let mut lexer = Lexer::new("= < > !=");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Operator("=".to_string()));
        assert_eq!(tokens[1], Token::Operator("<".to_string()));
        assert_eq!(tokens[2], Token::Operator(">".to_string()));
        assert_eq!(tokens[3], Token::Operator("!=".to_string()));
    }

    #[test]
    fn test_two_char_operators() {
        let mut lexer = Lexer::new("<= >= <>");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Operator("<=".to_string()));
        assert_eq!(tokens[1], Token::Operator(">=".to_string()));
        assert_eq!(tokens[2], Token::Operator("<".to_string())); // '<' then '>' as separate
    }

    // ========== 标点符号测试 ==========

    #[test]
    fn test_punctuation() {
        let mut lexer = Lexer::new("( ) , ; * .");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::LParen);
        assert_eq!(tokens[1], Token::RParen);
        assert_eq!(tokens[2], Token::Comma);
        assert_eq!(tokens[3], Token::Semicolon);
        assert_eq!(tokens[4], Token::Asterisk);
        assert_eq!(tokens[5], Token::Dot);
    }

    // ========== 边界条件测试 ==========

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize();
        // Only EOF token
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Eof);
    }

    #[test]
    fn test_whitespace_only() {
        let mut lexer = Lexer::new("   \t  \n  ");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Eof);
    }

    #[test]
    fn test_eof_at_end() {
        let mut lexer = Lexer::new("SELECT");
        let tokens = lexer.tokenize();
        assert_eq!(tokens.len(), 2); // SELECT + EOF
        assert_eq!(tokens.last().unwrap(), &Token::Eof);
    }

    // ========== 完整SQL语句测试 ==========

    #[test]
    fn test_select_statement_tokens() {
        let mut lexer = Lexer::new("SELECT * FROM users WHERE id = 1");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("SELECT".to_string()));
        assert_eq!(tokens[1], Token::Asterisk);
        assert_eq!(tokens[2], Token::Keyword("FROM".to_string()));
        assert_eq!(tokens[3], Token::Identifier("users".to_string()));
        assert_eq!(tokens[4], Token::Keyword("WHERE".to_string()));
        assert_eq!(tokens[5], Token::Identifier("id".to_string()));
        assert_eq!(tokens[6], Token::Operator("=".to_string()));
        assert_eq!(tokens[7], Token::Literal(Literal::Int(1)));
    }

    #[test]
    fn test_insert_statement_tokens() {
        let mut lexer = Lexer::new("INSERT INTO users (name, age) VALUES ('Alice', 25)");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("INSERT".to_string()));
        assert_eq!(tokens[1], Token::Keyword("INTO".to_string()));
        assert_eq!(tokens[2], Token::Identifier("users".to_string()));
        assert_eq!(tokens[3], Token::LParen);
        assert_eq!(tokens[4], Token::Identifier("name".to_string()));
        assert_eq!(tokens[5], Token::Comma);
        assert_eq!(tokens[6], Token::Identifier("age".to_string()));
        assert_eq!(tokens[7], Token::RParen);
        assert_eq!(tokens[8], Token::Keyword("VALUES".to_string()));
        assert_eq!(tokens[9], Token::LParen);
        assert_eq!(
            tokens[10],
            Token::Literal(Literal::String("Alice".to_string()))
        );
        assert_eq!(tokens[11], Token::Comma);
        assert_eq!(tokens[12], Token::Literal(Literal::Int(25)));
        assert_eq!(tokens[13], Token::RParen);
    }

    #[test]
    fn test_create_table_statement_tokens() {
        let mut lexer = Lexer::new("CREATE TABLE users (id INT, name VARCHAR)");
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0], Token::Keyword("CREATE".to_string()));
        assert_eq!(tokens[1], Token::Keyword("TABLE".to_string()));
        assert_eq!(tokens[2], Token::Identifier("users".to_string()));
        assert_eq!(tokens[3], Token::LParen);
        assert_eq!(tokens[4], Token::Identifier("id".to_string()));
        assert_eq!(tokens[5], Token::Keyword("INT".to_string()));
        assert_eq!(tokens[6], Token::Comma);
        assert_eq!(tokens[7], Token::Identifier("name".to_string()));
        assert_eq!(tokens[8], Token::Keyword("VARCHAR".to_string()));
        assert_eq!(tokens[9], Token::RParen);
    }
}
