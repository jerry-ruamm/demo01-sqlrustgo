/// SQLRustGo QPS 基准测试
/// 测试 DELETE / UPDATE / INSERT / SELECT 四种操作
/// 在 10,000 行规模下的每秒查询数 (QPS)
///
/// 运行方式:
///   cargo test --test qps_benchmark_test -- --ignored --nocapture
use sqlrustgo_1::storage::engine::Predicate;
use sqlrustgo_1::storage::MemoryStorage;
use sqlrustgo_1::storage::StorageEngine;
use sqlrustgo_1::types::{ColumnSchema, DataType, RecordBatch, Schema, Value};
use std::time::Instant;

const BENCHMARK_ITERATIONS: usize = 10_000;

fn make_schema() -> Schema {
    Schema::new(vec![
        ColumnSchema::new("id", DataType::Int, false),
        ColumnSchema::new("name", DataType::String, true),
        ColumnSchema::new("age", DataType::Int, true),
    ])
}

/// 批量插入 10,000 行数据
fn prepare_data(engine: &mut MemoryStorage) {
    engine.create_table("users", make_schema()).unwrap();
    let schema = make_schema();
    let mut batch = RecordBatch::new(schema);
    for i in 0..BENCHMARK_ITERATIONS {
        batch.add_row(vec![
            Value::Int(i as i64),
            Value::String(format!("bench_{}", i)),
            Value::Int((i % 60) as i64),
        ]);
    }
    let count = engine.write("users", batch).unwrap();
    assert_eq!(count, BENCHMARK_ITERATIONS);
}

// ==================== INSERT QPS 测试 ====================

#[test]
#[ignore]
fn test_qps_insert() {
    let mut engine = MemoryStorage::new();
    engine.create_table("users", make_schema()).unwrap();

    let start = Instant::now();
    for i in 0..BENCHMARK_ITERATIONS {
        let schema = make_schema();
        let mut batch = RecordBatch::new(schema);
        batch.add_row(vec![
            Value::Int(i as i64),
            Value::String(format!("bench_{}", i)),
            Value::Int((i % 60) as i64),
        ]);
        engine.write("users", batch).unwrap();
    }
    let elapsed = start.elapsed();
    let qps = BENCHMARK_ITERATIONS as f64 / elapsed.as_secs_f64();
    println!(
        "INSERT QPS: {} queries in {:.2?} ({:.2} qps)",
        BENCHMARK_ITERATIONS, elapsed, qps
    );
}

// ==================== SELECT QPS 测试 ====================

#[test]
#[ignore]
fn test_qps_simple_select() {
    let mut engine = MemoryStorage::new();
    prepare_data(&mut engine);

    let start = Instant::now();
    for i in 0..BENCHMARK_ITERATIONS {
        let pred = Predicate::Eq("id".to_string(), Value::Int(i as i64));
        let _ = engine.read("users", Some(pred)).unwrap();
    }
    let elapsed = start.elapsed();
    let qps = BENCHMARK_ITERATIONS as f64 / elapsed.as_secs_f64();
    println!(
        "SELECT QPS: {} queries in {:.2?} ({:.2} qps)",
        BENCHMARK_ITERATIONS, elapsed, qps
    );
}

// ==================== DELETE QPS 测试 ====================

#[test]
#[ignore]
fn test_qps_delete() {
    let mut engine = MemoryStorage::new();
    prepare_data(&mut engine);

    let start = Instant::now();
    for i in 0..BENCHMARK_ITERATIONS {
        let pred = Predicate::Eq("id".to_string(), Value::Int(i as i64));
        engine.delete("users", pred).unwrap();
    }
    let elapsed = start.elapsed();
    let qps = BENCHMARK_ITERATIONS as f64 / elapsed.as_secs_f64();
    println!(
        "DELETE QPS: {} queries in {:.2?} ({:.2} qps)",
        BENCHMARK_ITERATIONS, elapsed, qps
    );
}

// ==================== UPDATE QPS 测试 ====================

#[test]
#[ignore]
fn test_qps_update() {
    let mut engine = MemoryStorage::new();
    prepare_data(&mut engine);

    let start = Instant::now();
    let schema = make_schema();
    for i in 0..BENCHMARK_ITERATIONS {
        let pred = Predicate::Eq("id".to_string(), Value::Int(i as i64));
        // Step 1: read the matching row to get its name
        let batch = engine.read("users", Some(pred)).unwrap();
        if batch.row_count() > 0 {
            let old_name = match &batch.rows()[0][1] {
                Value::String(s) => s.clone(),
                _ => format!("bench_{}", i),
            };
            // Step 2: delete old row
            engine
                .delete(
                    "users",
                    Predicate::Eq("id".to_string(), Value::Int(i as i64)),
                )
                .unwrap();
            // Step 3: insert updated row
            let mut new_batch = RecordBatch::new(schema.clone());
            new_batch.add_row(vec![
                Value::Int(i as i64),
                Value::String(format!("updated_{}", old_name)),
                Value::Int(((i + 1) % 60) as i64),
            ]);
            engine.write("users", new_batch).unwrap();
        }
    }
    let elapsed = start.elapsed();
    let qps = BENCHMARK_ITERATIONS as f64 / elapsed.as_secs_f64();
    println!(
        "UPDATE QPS: {} queries in {:.2?} ({:.2} qps)",
        BENCHMARK_ITERATIONS, elapsed, qps
    );
}
