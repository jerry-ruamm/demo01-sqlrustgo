# Executor 模块设计文档

## 1. 模块概述

### 1.1 模块职责

Executor模块基于火山模型(Volcano Model)实现查询执行引擎，负责物理执行计划的实际执行。通过算子树的方式，逐个处理数据。

### 1.2 核心功能

| 功能 | 说明 |
|------|------|
| **执行查询** | 根据物理执行计划执行查询 |
| **管理算子** | 创建和管理执行算子树 |
| **处理结果** | 处理和返回执行结果 |
| **资源管理** | 管理执行过程中的内存和资源 |

### 1.3 设计原则

- **火山模型**：采用标准的Volcano执行模型
- **向量化执行**：支持批量数据处理提高性能
- **算子复用**：算子设计独立可复用
- **错误隔离**：执行错误不影响其他查询

---

## 2. OOA分析

### 2.1 用例图

```mermaid
graph LR
    subgraph 参与者
        Client["客户端/Planner"]
    end
    
    subgraph Executor模块
        Execute["执行查询<br/>(Execute Query)"]
        Operators["管理算子<br/>(Operator Tree)"]
        Results["处理结果<br/>(Result Sets)"]
        Resource["资源管理<br/>(Resource)"]
    end
    
    Client --> Execute
    Client --> Operators
    Client --> Results
    Client --> Resource
```

### 2.2 概念类图

```mermaid
classDiagram
    class "执行引擎" as Executor {
        +execute(plan) ResultSet
        +cancel(query_id)
    }
    
    class "执行算子" as Operator {
        <<interface>>
        +open()
        +next() Option~RecordBatch~
        +close()
    }
    
    class "执行计划" as PhysicalPlan {
        +OperatorType type
        +List~PhysicalPlan~ children
    }
    
    class "结果集" as ResultSet {
        +Schema schema
        +List~RecordBatch~ batches
        +row_count() usize
    }
    
    class "执行上下文" as ExecutionContext {
        +QueryId query_id
        +u64 memory_used
        +HashMap config
    }
    
    Executor "1" --> "1" PhysicalPlan : 接收
    Executor "1" --> "*" Operator : 构建
    Executor "1" --> "1" ExecutionContext : 使用
    Operator "*" --> "1" ResultSet : 产生
```

### 2.3 活动图

```mermaid
stateDiagram-v2
    [*] --> 接收物理执行计划
    
    接收物理执行计划 --> 构建执行算子树
    构建执行算子树 --> 初始化执行上下文
    
    初始化执行上下文 --> 调用算子open()
    调用算子open() --> 循环调用算子next()
    
    循环调用算子next() --> 有数据?:
    有数据? --> 处理记录批次 : 是
    处理记录批次 --> 收集执行结果
    收集执行结果 --> 循环调用算子next()
    
    有数据? --> 调用算子close() : 否
    调用算子close() --> 清理执行资源
    清理执行资源 --> 返回结果集
    
    返回结果集 --> [*]
```

---

## 3. OOD设计

### 3.1 设计类图

```mermaid
classDiagram
    class Executor {
        <<trait>>
        +execute(plan: &PhysicalPlan) Result~ResultSet~
    }
    
    class Operator {
        <<trait>>
        +open() Result~()~
        +next() Result~Option~RecordBatch~~
        +close() Result~()~
        +children() &[Box~dyn Operator~]
    }
    
    class SimpleExecutor {
        -context: ExecutionContext
        +execute(plan: &PhysicalPlan) Result~ResultSet~
        +build_operator(plan: &PhysicalPlan) Result~Box~dyn Operator~~
    }
    
    class ScanOperator {
        -storage: Arc~dyn StorageEngine~
        -table_name: String
        -columns: Vec~String~
        -predicate: Option~Expression~
        -cursor: usize
        +open() Result~()~
        +next() Result~Option~RecordBatch~~
        +close() Result~()~
    }
    
    class ProjectOperator {
        -child: Box~dyn Operator~
        -projections: Vec~Expression~
        +open() Result~()~
        +next() Result~Option~RecordBatch~~
        +close() Result~()~
    }
    
    class FilterOperator {
        -child: Box~dyn Operator~
        -predicate: Expression
        +open() Result~()~
        +next() Result~Option~RecordBatch~~
        +close() Result~()~
    }
    
    class HashJoinOperator {
        -left: Box~dyn Operator~
        -right: Box~dyn Operator~
        -join_type: JoinType
        -hash_table: HashMap
        +open() Result~()~
        +next() Result~Option~RecordBatch~~
        +close() Result~()~
    }
    
    class HashAggregateOperator {
        -child: Box~dyn Operator~
        -group_keys: Vec~Expression~
        -aggregates: Vec~AggregateExpr~
        -hash_table: HashMap
        +open() Result~()~
        +next() Result~Option~RecordBatch~~
        +close() Result~()~
    }
    
    class SortOperator {
        -child: Box~dyn Operator~
        -order_by: Vec~OrderByExpr~
        -buffer: Vec~RecordBatch~
        -sorted: bool
        +open() Result~()~
        +next() Result~Option~RecordBatch~~
        +close() Result~()~
    }
    
    class LimitOperator {
        -child: Box~dyn Operator~
        -limit: usize
        -offset: usize
        -returned: usize
        +open() Result~()~
        +next() Result~Option~RecordBatch~~
        +close() Result~()~
    }
    
    class ExecutionContext {
        -query_id: Uuid
        -memory_limit: usize
        -memory_used: usize
        -timeout: Duration
        -start_time: Instant
        +allocate_memory(size: usize) Result~()~
        +is_timeout() bool
    }
    
    Executor <|.. SimpleExecutor
    Operator <|.. ScanOperator
    Operator <|.. ProjectOperator
    Operator <|.. FilterOperator
    Operator <|.. HashJoinOperator
    Operator <|.. HashAggregateOperator
    Operator <|.. SortOperator
    Operator <|.. LimitOperator
    
    SimpleExecutor --> Operator : 构建和执行
    SimpleExecutor --> ExecutionContext : 使用
    
    ScanOperator --> StorageEngine : 依赖
    ProjectOperator --> Operator : 子算子
    FilterOperator --> Operator : 子算子
    HashJoinOperator --> Operator : 左/右子算子
    HashAggregateOperator --> Operator : 子算子
    SortOperator --> Operator : 子算子
    LimitOperator --> Operator : 子算子
```

### 3.2 顺序图

```mermaid
sequenceDiagram
    participant Client as 客户端
    participant Executor as SimpleExecutor
    participant Context as ExecutionContext
    participant Scan as ScanOperator
    participant Filter as FilterOperator
    participant Project as ProjectOperator
    participant Storage as StorageEngine
    
    Client->>Executor: execute(physical_plan)
    
    Executor->>Context: new() 创建执行上下文
    
    Executor->>Executor: build_operator(plan)
    
    Executor->>Scan: new(table, columns, predicate)
    Executor->>Filter: new(scan, predicate)
    Executor->>Project: new(filter, projections)
    
    Executor->>Project: open()
    Project->>Filter: open()
    Filter->>Scan: open()
    Scan->>Storage: begin_scan(table)
    Storage-->>Scan: Ok
    
    loop next() 拉取数据
        Executor->>Project: next()
        Project->>Filter: next()
        Filter->>Scan: next()
        Scan->>Storage: read_batch()
        Storage-->>Scan: Some(RecordBatch)
        Scan-->>Filter: Some(RecordBatch)
        Filter->>Filter: 应用谓词过滤
        Filter-->>Project: Some(RecordBatch)
        Project->>Project: 应用投影表达式
        Project-->>Executor: Some(RecordBatch)
    end
    
    Executor->>Project: next()
    Project->>Filter: next()
    Filter->>Scan: next()
    Scan-->>Filter: None 数据结束
    Filter-->>Project: None
    Project-->>Executor: None
    
    Executor->>Project: close()
    Project->>Filter: close()
    Filter->>Scan: close()
    
    Executor-->>Client: ResultSet
```

### 3.3 状态图

```mermaid
stateDiagram-v2
    [*] --> Idle
    
    Idle --> Building : 开始执行
    Building --> Opening : 算子树构建完成
    
    state Opening {
        [*] --> OpenRoot
        OpenRoot --> OpenChildren
        OpenChildren --> Opened : 所有算子open成功
    }
    
    Opened --> Executing : 开始执行
    
    state Executing {
        [*] --> Pulling
        Pulling --> Processing : 有数据
        Processing --> Pulling : 继续拉取
        Pulling --> Finished : 无数据
    }
    
    Finished --> Closing : 执行结束
    Closing --> Closed : 所有算子close完成
    
    Closed --> Idle : 重置状态
    
    note right of Executing: 火山模型<br/>由顶向下pull数据
```

### 3.4 组件图

```mermaid
graph TD
    subgraph Executor组件
        Engine["执行引擎<br/>(Executor)"]
        subgraph Operators
            Scan["Scan算子"]
            Filter["Filter算子"]
            Project["Project算子"]
            Join["Join算子"]
            Agg["Aggregate算子"]
        end
        Context["执行上下文<br/>(Context)"]
    end
    
    subgraph Storage组件
        Engine["存储引擎"]
    end
    
    subgraph Common组件
        Batch["RecordBatch"]
        Expr["表达式计算"]
    end
    
    Engine --> Scan
    Engine --> Filter
    Engine --> Project
    Engine --> Join
    Engine --> Agg
    Engine --> Context
    
    Scan --> Engine : 依赖
    Operators --> Batch : 产出
    Operators --> Expr : 使用
```

---

## 4. 核心接口设计

### 4.1 Operator Trait（火山模型核心）

```rust
pub trait Operator: Send + Sync {
    fn open(&mut self) -> Result<(), ExecError>;
    
    fn next(&mut self) -> Result<Option<RecordBatch>, ExecError>;
    
    fn close(&mut self) -> Result<(), ExecError>;
    
    fn children(&self) -> &[Box<dyn Operator>] {
        &[]
    }
    
    fn name(&self) -> &str;
}
```

### 4.2 Executor Trait

```rust
pub trait Executor {
    fn execute(&mut self, plan: &PhysicalPlan) -> Result<ResultSet, ExecError>;
    
    fn execute_stream(&mut self, plan: &PhysicalPlan) 
        -> Box<dyn Iterator<Item = Result<RecordBatch, ExecError>>>;
}
```

### 4.3 ExecutionContext

```rust
pub struct ExecutionContext {
    query_id: Uuid,
    memory_limit: usize,
    memory_used: usize,
    batch_size: usize,
    start_time: Instant,
    timeout: Option<Duration>,
}

impl ExecutionContext {
    pub fn allocate_memory(&mut self, size: usize) -> Result<(), ExecError> {
        if self.memory_used + size > self.memory_limit {
            return Err(ExecError::MemoryLimitExceeded);
        }
        self.memory_used += size;
        Ok(())
    }
    
    pub fn is_timeout(&self) -> bool {
        self.timeout.map_or(false, |t| self.start_time.elapsed() > t)
    }
}
```

---

## 5. 算子设计

### 5.1 核心算子列表

| 算子 | 说明 | 实现复杂度 |
|------|------|-----------|
| **Scan** | 表扫描算子 | 低 |
| **IndexScan** | 索引扫描算子 | 中 |
| **Filter** | 过滤算子 | 低 |
| **Project** | 投影算子 | 低 |
| **HashJoin** | Hash连接算子 | 高 |
| **NestedLoopJoin** | 嵌套循环连接算子 | 中 |
| **HashAggregate** | Hash聚合算子 | 高 |
| **Sort** | 排序算子 | 中 |
| **Limit** | 分页算子 | 低 |

### 5.2 向量化执行

```rust
// 每次处理1024行的批次，而不是单行
pub const DEFAULT_BATCH_SIZE: usize = 1024;

// RecordBatch包含多行数据，支持SIMD优化
pub struct RecordBatch {
    schema: Schema,
    columns: Vec<ColumnArray>,
    row_count: usize,
}
```

---

## 6. 错误处理设计

### 6.1 错误类型

```rust
#[derive(Debug)]
pub enum ExecError {
    OperatorError(String),
    StorageError(StorageError),
    ExpressionError(ExprError),
    MemoryLimitExceeded,
    QueryTimeout,
    Canceled,
    InternalError(String),
}
```

### 6.2 错误恢复策略

1. **原子关闭**：确保所有算子正确关闭释放资源
2. **错误隔离**：单个查询错误不影响其他查询
3. **详细日志**：记录完整的错误栈和上下文信息

---

## 7. 测试策略

| 测试类型 | 测试内容 |
|---------|---------|
| **算子单元测试** | 每个算子的open/next/close |
| **集成测试** | 完整算子树执行 |
| **TPC-H测试** | 标准基准测试验证正确性 |
| **内存测试** | 验证内存使用不超过限制 |
| **并发测试** | 多查询并发执行 |
| **性能测试** | 单个算子和整体执行性能 |
