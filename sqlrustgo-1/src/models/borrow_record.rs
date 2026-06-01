#[allow(dead_code)]
pub trait FineCalculator {
    fn calculate(&self, days_overdue: u32) -> f64;
}

#[allow(dead_code)]
pub struct StandardFineCalculator {
    daily_rate: f64,
}

impl StandardFineCalculator {
    pub fn new(daily_rate: f64) -> Self {
        StandardFineCalculator { daily_rate }
    }
}

impl FineCalculator for StandardFineCalculator {
    fn calculate(&self, days_overdue: u32) -> f64 {
        days_overdue as f64 * self.daily_rate
    }
}

#[allow(dead_code)]
pub struct BorrowRecord {
    pub id: String,
    pub user_id: String,
    pub book_isbn: String,
    pub borrow_date: String,
    pub due_date: String,
    pub return_date: Option<String>,
    fine_calculator: Box<dyn FineCalculator>,
}

#[allow(dead_code)]
impl BorrowRecord {
    pub fn new(
        id: String,
        user_id: String,
        book_isbn: String,
        borrow_date: String,
        due_date: String,
        fine_calculator: Box<dyn FineCalculator>,
    ) -> Self {
        BorrowRecord {
            id,
            user_id,
            book_isbn,
            borrow_date,
            due_date,
            return_date: None,
            fine_calculator,
        }
    }

    pub fn is_overdue(&self) -> bool {
        self.return_date.is_none()
    }

    pub fn calculate_fine(&self) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_fine_calculator() {
        let calc = StandardFineCalculator::new(0.5);
        assert_eq!(calc.calculate(10), 5.0);
        assert_eq!(calc.calculate(0), 0.0);
        assert_eq!(calc.calculate(7), 3.5);
    }

    #[test]
    fn test_standard_fine_calculator_zero_rate() {
        let calc = StandardFineCalculator::new(0.0);
        assert_eq!(calc.calculate(100), 0.0);
    }

    #[test]
    fn test_borrow_record_new() {
        let calc = Box::new(StandardFineCalculator::new(0.5));
        let record = BorrowRecord::new(
            "BR001".to_string(),
            "U001".to_string(),
            "ISBN-001".to_string(),
            "2026-05-01".to_string(),
            "2026-06-01".to_string(),
            calc,
        );
        assert_eq!(record.id, "BR001");
        assert_eq!(record.user_id, "U001");
        assert!(record.return_date.is_none());
        assert!(record.is_overdue()); // Not returned yet
        assert_eq!(record.calculate_fine(), 0.0);
    }

    #[test]
    fn test_fine_calculator_trait_object() {
        let calc: Box<dyn FineCalculator> = Box::new(StandardFineCalculator::new(1.0));
        assert_eq!(calc.calculate(5), 5.0);
    }
}
