#[derive(Debug, Clone)]
pub enum LogicalPlan {
    Scan(Scan),
    Projection(Projection),
    Selection(Selection),
}

#[derive(Debug, Clone)]
pub struct Scan {
    pub table: String,
}

#[derive(Debug, Clone)]
pub struct Projection {
    pub columns: Vec<String>,
    pub input: Box<LogicalPlan>,
}

#[derive(Debug, Clone)]
pub struct Selection {
    pub predicate: String,
    pub input: Box<LogicalPlan>,
}

pub struct LogicalPlanner {}

#[allow(clippy::new_without_default)]
impl LogicalPlanner {
    pub fn new() -> Self {
        LogicalPlanner {}
    }

    pub fn plan(
        &self,
        stmt: crate::parser::Statement,
    ) -> Result<LogicalPlan, super::error::PlanError> {
        match stmt {
            crate::parser::Statement::Select(select) => {
                let table_name = select.from.ok_or(super::error::PlanError::MissingTable)?;
                let mut plan = LogicalPlan::Scan(Scan { table: table_name });

                if select.where_clause.is_some() {
                    plan = LogicalPlan::Selection(Selection {
                        predicate: "predicate".to_string(),
                        input: Box::new(plan),
                    });
                }

                let columns = vec!["*".to_string()];
                plan = LogicalPlan::Projection(Projection {
                    columns,
                    input: Box::new(plan),
                });

                Ok(plan)
            }
            _ => Ok(LogicalPlan::Scan(Scan {
                table: "default".to_string(),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{SelectStatement, Statement};

    #[test]
    fn test_logical_planner_new() {
        let planner = LogicalPlanner::new();
        let _ = planner; // Verify construction
    }

    #[test]
    fn test_plan_select_statement() {
        let planner = LogicalPlanner::new();
        let stmt = Statement::Select(SelectStatement {
            columns: vec![],
            from: Some("users".to_string()),
            where_clause: None,
        });

        let result = planner.plan(stmt);
        assert!(result.is_ok());
        match result.unwrap() {
            LogicalPlan::Projection(proj) => match *proj.input {
                LogicalPlan::Scan(scan) => assert_eq!(scan.table, "users"),
                _ => panic!("Expected Scan inside Projection"),
            },
            _ => panic!("Expected Projection plan"),
        }
    }

    #[test]
    fn test_plan_select_with_where() {
        let planner = LogicalPlanner::new();
        use crate::parser::ast::Expression;
        let stmt = Statement::Select(SelectStatement {
            columns: vec![],
            from: Some("users".to_string()),
            where_clause: Some(Expression::Column("id".to_string())),
        });

        let result = planner.plan(stmt);
        assert!(result.is_ok());
        match result.unwrap() {
            LogicalPlan::Projection(proj) => {
                match *proj.input {
                    LogicalPlan::Selection(_) => {} // Selection inside Projection
                    _ => panic!("Expected Selection"),
                }
            }
            _ => panic!("Expected Projection"),
        }
    }

    #[test]
    fn test_plan_select_without_from_errors() {
        let planner = LogicalPlanner::new();
        let stmt = Statement::Select(SelectStatement {
            columns: vec![],
            from: None,
            where_clause: None,
        });

        let result = planner.plan(stmt);
        assert!(result.is_err());
    }

    #[test]
    fn test_plan_non_select() {
        // INSERT/CREATE TABLE go to default branch
        let planner = LogicalPlanner::new();
        use crate::parser::ast::InsertStatement;
        let stmt = Statement::Insert(InsertStatement {
            table: "users".to_string(),
            columns: vec![],
            values: vec![],
        });

        let result = planner.plan(stmt);
        assert!(result.is_ok());
    }
}
