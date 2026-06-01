#[allow(dead_code)]
pub trait User {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn borrow_limit(&self) -> u32;
}

pub struct Student {
    pub student_id: String,
    pub name: String,
}

impl User for Student {
    fn id(&self) -> &str {
        &self.student_id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn borrow_limit(&self) -> u32 {
        5
    }
}

#[allow(dead_code)]
pub struct Teacher {
    pub teacher_id: String,
    pub name: String,
}

impl User for Teacher {
    fn id(&self) -> &str {
        &self.teacher_id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn borrow_limit(&self) -> u32 {
        10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_creation() {
        let student = Student {
            student_id: "S001".to_string(),
            name: "张三".to_string(),
        };
        assert_eq!(student.id(), "S001");
        assert_eq!(student.name(), "张三");
        assert_eq!(student.borrow_limit(), 5);
    }

    #[test]
    fn test_student_borrow_limit() {
        let student = Student {
            student_id: "S002".to_string(),
            name: "李四".to_string(),
        };
        assert_eq!(student.borrow_limit(), 5);
    }

    #[test]
    fn test_teacher_creation() {
        let teacher = Teacher {
            teacher_id: "T001".to_string(),
            name: "王老师".to_string(),
        };
        assert_eq!(teacher.id(), "T001");
        assert_eq!(teacher.name(), "王老师");
        assert_eq!(teacher.borrow_limit(), 10);
    }

    #[test]
    fn test_user_trait_object() {
        let student = Student {
            student_id: "S003".to_string(),
            name: "测试".to_string(),
        };
        let users: Vec<&dyn User> = vec![&student];
        assert_eq!(users[0].borrow_limit(), 5);
    }
}
