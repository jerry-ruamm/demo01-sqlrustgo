#[allow(dead_code)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub publisher: String,
    pub available: bool,
}

impl Book {
    pub fn new(isbn: String, title: String, author: String, publisher: String) -> Self {
        Book {
            isbn,
            title,
            author,
            publisher,
            available: true,
        }
    }

    pub fn borrow(&mut self) -> Result<(), String> {
        if self.available {
            self.available = false;
            Ok(())
        } else {
            Err("Book is not available".to_string())
        }
    }

    #[allow(dead_code)]
    pub fn return_book(&mut self) {
        self.available = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_new() {
        let book = Book::new(
            "978-7-111-63975-6".to_string(),
            "数据库系统实现".to_string(),
            "Hector Garcia-Molina".to_string(),
            "机械工业出版社".to_string(),
        );
        assert_eq!(book.isbn, "978-7-111-63975-6");
        assert_eq!(book.title, "数据库系统实现");
        assert!(book.available);
    }

    #[test]
    fn test_book_borrow_success() {
        let mut book = Book::new(
            "ISBN-001".to_string(),
            "Rust编程".to_string(),
            "作者".to_string(),
            "出版社".to_string(),
        );
        assert!(book.borrow().is_ok());
        assert!(!book.available);
    }

    #[test]
    fn test_book_borrow_twice_fails() {
        let mut book = Book::new(
            "ISBN-002".to_string(),
            "测试书".to_string(),
            "作者".to_string(),
            "出版社".to_string(),
        );
        assert!(book.borrow().is_ok());
        assert!(book.borrow().is_err());
    }

    #[test]
    fn test_book_return() {
        let mut book = Book::new(
            "ISBN-003".to_string(),
            "可归还书".to_string(),
            "作者".to_string(),
            "出版社".to_string(),
        );
        book.borrow().unwrap();
        assert!(!book.available);
        book.return_book();
        assert!(book.available);
    }

    #[test]
    fn test_book_reborrow_after_return() {
        let mut book = Book::new(
            "ISBN-004".to_string(),
            "重复借阅".to_string(),
            "作者".to_string(),
            "出版社".to_string(),
        );
        book.borrow().unwrap();
        book.return_book();
        assert!(book.borrow().is_ok());
    }
}
