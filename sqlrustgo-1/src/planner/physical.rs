#[derive(Debug, Clone)]
pub enum PhysicalPlan {
    TableScan(TableScan),
    Filter(Filter),
    Project(Project),
}

#[derive(Debug, Clone)]
pub struct TableScan {
    pub table: String,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub columns: Vec<String>,
    pub input: Box<PhysicalPlan>,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub predicate: String,
    pub input: Box<PhysicalPlan>,
}

pub struct PhysicalPlanner {}

#[allow(clippy::new_without_default)]
impl PhysicalPlanner {
    pub fn new() -> Self {
        PhysicalPlanner {}
    }

    pub fn plan(
        &self,
        logical: super::logical::LogicalPlan,
    ) -> Result<PhysicalPlan, super::error::PlanError> {
        match logical {
            super::LogicalPlan::Scan(scan) => {
                Ok(PhysicalPlan::TableScan(TableScan { table: scan.table }))
            }
            super::LogicalPlan::Projection(proj) => {
                let input = self.plan(*proj.input)?;
                Ok(PhysicalPlan::Project(Project {
                    columns: proj.columns,
                    input: Box::new(input),
                }))
            }
            super::LogicalPlan::Selection(sel) => {
                let input = self.plan(*sel.input)?;
                Ok(PhysicalPlan::Filter(Filter {
                    predicate: sel.predicate,
                    input: Box::new(input),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planner::logical::{LogicalPlan, Projection, Scan, Selection};

    #[test]
    fn test_physical_planner_new() {
        let planner = PhysicalPlanner::new();
        let _ = planner;
    }

    #[test]
    fn test_plan_scan_to_table_scan() {
        let planner = PhysicalPlanner::new();
        let logical = LogicalPlan::Scan(Scan {
            table: "users".to_string(),
        });
        let result = planner.plan(logical).unwrap();
        match result {
            PhysicalPlan::TableScan(ts) => assert_eq!(ts.table, "users"),
            _ => panic!("Expected TableScan"),
        }
    }

    #[test]
    fn test_plan_projection() {
        let planner = PhysicalPlanner::new();
        let logical = LogicalPlan::Projection(Projection {
            columns: vec!["id".to_string()],
            input: Box::new(LogicalPlan::Scan(Scan {
                table: "users".to_string(),
            })),
        });
        let result = planner.plan(logical).unwrap();
        match result {
            PhysicalPlan::Project(proj) => {
                assert_eq!(proj.columns, vec!["id"]);
                match *proj.input {
                    PhysicalPlan::TableScan(ts) => assert_eq!(ts.table, "users"),
                    _ => panic!("Expected TableScan in Project"),
                }
            }
            _ => panic!("Expected Project"),
        }
    }

    #[test]
    fn test_plan_selection() {
        let planner = PhysicalPlanner::new();
        let logical = LogicalPlan::Selection(Selection {
            predicate: "x > 1".to_string(),
            input: Box::new(LogicalPlan::Scan(Scan {
                table: "users".to_string(),
            })),
        });
        let result = planner.plan(logical).unwrap();
        match result {
            PhysicalPlan::Filter(filter) => {
                assert_eq!(filter.predicate, "x > 1");
                match *filter.input {
                    PhysicalPlan::TableScan(_) => {}
                    _ => panic!("Expected TableScan in Filter"),
                }
            }
            _ => panic!("Expected Filter"),
        }
    }
}
