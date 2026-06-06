use crate::types::Value;

#[derive(Debug, Clone)]
pub enum Statement {
    Select(SelectStatement),
    Insert(InsertStatement),
    CreateTable(CreateTableStatement),
}

#[derive(Debug, Clone)]
pub struct SelectStatement {
    pub columns: Vec<Expression>,
    pub from: Option<String>,
    pub where_clause: Option<Expression>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub order_by: Vec<OrderByClause>,
}

#[derive(Debug, Clone)]
pub struct OrderByClause {
    pub column: String,
    pub direction: OrderDirection,
}

#[derive(Debug, Clone)]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct InsertStatement {
    pub table: String,
    pub columns: Vec<String>,
    pub values: Vec<Vec<Expression>>,
}

#[derive(Debug, Clone)]
pub struct CreateTableStatement {
    pub table: String,
    pub columns: Vec<ColumnDef>,
}

#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Clone)]
pub enum DataType {
    Int,
    Float,
    Boolean,
    String,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Column(String),
    Literal(Value),
    Binary(Box<Expression>, Operator, Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
    Add,
    Sub,
    Mul,
    Div,
}
