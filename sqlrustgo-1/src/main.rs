mod models;

use sqlrustgo_1::storage::{MemoryStorage, StorageEngine};

fn main() {
    println!("========================================");
    println!("   SQLRustGo v1.0 数据库系统演示");
    println!("========================================");

    println!("\n[1/5] 初始化存储引擎...");
    let mut storage = MemoryStorage::new();
    println!("✓ MemoryStorage 初始化完成");

    println!("\n[2/5] 创建表 users...");
    {
        use sqlrustgo_1::types::{ColumnSchema, DataType, Schema};
        let columns = vec![
            ColumnSchema::new("id", DataType::Int, false),
            ColumnSchema::new("name", DataType::String, false),
            ColumnSchema::new("age", DataType::Int, true),
        ];
        let schema = Schema::new(columns);
        storage.create_table("users", schema).unwrap();
        println!("✓ 表 users 创建成功 (id INT, name VARCHAR, age INT)");
    }

    println!("\n[3/5] 测试SQL词法分析器...");
    {
        use sqlrustgo_1::parser::Lexer;
        let sql = "SELECT * FROM users";
        let mut lexer = Lexer::new(sql);
        let tokens = lexer.tokenize();
        println!("✓ SQL词法分析成功: {}", sql);
        println!("  Token数量: {}", tokens.len() - 1);
    }

    println!("\n[4/5] 执行 SELECT 查询...");
    {
        let batch = storage.read("users", None).unwrap();
        println!("✓ 执行查询成功: SELECT * FROM users");
        println!("  读取到 {} 行数据", batch.row_count());
    }

    println!("\n[5/5] 查看模块结构...");
    println!("  ✓ parser/   - 解析层 (Lexer, Parser, AST)");
    println!("  ✓ planner/  - 规划层 (逻辑计划, 物理计划)");
    println!("  ✓ executor/ - 执行层 (执行引擎, 算子)");
    println!("  ✓ storage/  - 存储层 (存储引擎接口, 内存存储)");
    println!("  ✓ types/    - 通用类型 (Value, Schema)");

    println!("\n========================================");
    println!("   SQLRustGo v1.0 架构骨架构建完成!");
    println!("========================================");

    library_system_demo();
}

fn library_system_demo() {
    println!("\n\n--- 图书借阅系统演示 ---");
    use models::{Book, BorrowRecord, StandardFineCalculator, Student, User};

    let student = Student {
        student_id: "S202442020122".to_string(),
        name: "姚汶辰".to_string(),
    };

    let mut book = Book::new(
        "9787111639756".to_string(),
        "数据库系统实现".to_string(),
        "Hector Garcia-Molina".to_string(),
        "机械工业出版社".to_string(),
    );

    let borrow_date = "2026-04-18".to_string();
    let due_date = "2026-05-18".to_string();

    let fine_calculator = Box::new(StandardFineCalculator::new(0.5));

    let borrow_record = BorrowRecord::new(
        "BR001".to_string(),
        student.id().to_string(),
        book.isbn.clone(),
        borrow_date,
        due_date,
        fine_calculator,
    );

    book.borrow().unwrap();
    println!("✓ 图书借阅成功: {}", book.title);
    println!("  借阅人: {} ({})", student.name(), student.id());
    println!("  借阅记录: {}", borrow_record.id);
}
