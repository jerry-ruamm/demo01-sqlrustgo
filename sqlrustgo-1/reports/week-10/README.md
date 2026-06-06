# 第10周实验报告：Harness治理实战 — DELETE/UPDATE性能优化

---

## 实验基本信息

| 项目 | 内容 |
|------|------|
| **实验名称** | SQLRustGo Harness治理实战：DELETE/UPDATE性能优化 |
| **实验周次** | 第 10 周 |
| **实验日期** | 2026 年 6 月 6 日 |
| **学生姓名** | 姚汶辰 |
| **学号** | 202442020122 |
| **班级** | 24级软件工程1班 |
| **指导教师** | 李莹 |

---

## 一、实验目标

1. 理解 Harness 治理的三层模型（提示词层 → 上下文层 → Harness 层）
2. 能够运行 QPS 基准测试并分析性能瓶颈
3. 能够使用 AI 辅助定位性能问题根因
4. 能够模拟 Gate 检查流程（BP1 静态检查 → BP2 行为检查 → 拦截 → 修复 → 通过）
5. 能够绘制 Harness 治理闭环图

---

## 二、实验环境

### 2.1 硬件环境

| 项目 | 配置 |
|------|------|
| 计算机型号 | Lenovo Legion R9000P |
| CPU | AMD Ryzen 9 7940HX 16核32线程 |
| 内存 | 16GB DDR5 |
| 硬盘 | WD PC SN560 1TB NVMe SSD |
| GPU | NVIDIA GeForce RTX 4070 Laptop GPU |

### 2.2 软件环境

| 软件 | 版本 |
|------|------|
| 操作系统 | Windows 11 22H2 |
| Rust | 1.94.0 stable (x86_64-pc-windows-gnu) |
| Git | 2.44.0 |
| AI 工具 | Claude Code / TRAE IDE |

---

## 三、实验背景

### SQLRustGo v3.0.0 的性能挑战

在 v3.0.0 版本中，DELETE 和 UPDATE 操作的性能是已知的痛点。简单的内存存储引擎使用 `Vec<Vec<Value>>` 存储数据，DELETE 通过 `retain()` 逐行扫描，UPDATE 需要"读 → 删 → 写"三步完成。在 10,000 行规模下，这些操作的表现如何？能否达到 E-09 硬性地板（≥10,000 QPS）？

本实验将回答这些问题，并通过 Harness 治理框架来约束优化方向。

### E-09 硬性地板

| 操作 | E-09 地板 | 说明 |
|------|----------|------|
| DELETE | ≥ 10,000 QPS | 每秒至少删除 10,000 行 |
| UPDATE | ≥ 10,000 QPS | 每秒至少更新 10,000 行 |
| INSERT | ≥ 5,000 QPS | 参考值 |
| SELECT | ≥ 20,000 QPS | 参考值 |

---

## 四、实验内容

### 步骤0：切换到正确分支

```bash
cd D:/workcourse/workware/demo01
git checkout develop/v3.0.0
```

本实验在 `develop/v3.0.0` 分支上进行，该分支代表 v3.0.0 版本的开发主线。

---

### 步骤1：运行 QPS 基准测试

#### 1.1 基准测试设计

编写了 `tests/qps_benchmark_test.rs`，包含四个 `#[ignore]` 测试：

| 测试函数 | 操作 | 测试方式 |
|----------|------|----------|
| `test_qps_insert` | INSERT | 逐条插入 10,000 行，每行通过 `RecordBatch` + `write()` |
| `test_qps_simple_select` | SELECT | 在 10,000 行数据上逐条 `WHERE id = ?` 查询 |
| `test_qps_delete` | DELETE | 在 10,000 行数据上逐条 `WHERE id = ?` 删除 |
| `test_qps_update` | UPDATE | 在 10,000 行数据上逐条"读-删-写"模拟更新 |

#### 1.2 运行命令

```bash
cargo test --test qps_benchmark_test test_qps_insert -- --ignored --nocapture
cargo test --test qps_benchmark_test test_qps_simple_select -- --ignored --nocapture
cargo test --test qps_benchmark_test test_qps_delete -- --ignored --nocapture
cargo test --test qps_benchmark_test test_qps_update -- --ignored --nocapture
```

#### 1.3 性能基线数据

| 操作 | QPS | 与 E-09 地板比较 | 结论 |
|------|-----|-----------------|------|
| **INSERT** | **1,213,209** | 地板: 5,000 | ✅ 大幅超越 |
| **SELECT** | **3,294** | 地板: 20,000 | ❌ 不达标 |
| **DELETE** | **5,105** | 地板: 10,000 | ❌ **不达标** |
| **UPDATE** | **1,384** | 地板: 10,000 | ❌ **不达标** |

**关键发现**：

- INSERT 速度极快（120万 QPS），因为只是 `Vec::push()`，几乎没有开销
- SELECT 只有 3,294 QPS——每次查询都要扫描全部 10,000 行并逐行评估谓词
- DELETE 只有 5,105 QPS——`Vec::retain()` 虽然也是 O(n)，但内部需要移动元素
- UPDATE 最慢，只有 1,384 QPS——每次更新需要 3 次存储操作（read + delete + write）

> 📊 基准测试完整输出：`reports/week-10/screenshots/benchmark_output.txt`

---

### 步骤2：AI 辅助根因分析

#### 2.1 向 AI 提问

```
SQLRustGo 的 DELETE 操作只有 5,105 QPS，而 INSERT 有 1,213,209 QPS。
为什么 DELETE 比 INSERT 慢 237 倍？可能的原因有哪些？

请从以下角度分析：
1. DELETE 操作在存储引擎内部需要做哪些额外工作（vs INSERT）？
2. 当前 Vec<Vec<Value>> 实现中可能有哪些性能瓶颈？
3. 给出 2-3 个可能的优化方向。
```

#### 2.2 AI 分析结果

AI 指出了以下核心问题：

**问题1：全表扫描（O(n) 遍历）**

`MemoryStorage::delete()` 的当前实现：

```rust
fn delete(&mut self, table: &str, predicate: Predicate) -> Result<usize, StorageError> {
    // ...
    self.data.get_mut(table).unwrap()
        .retain(|row| !Self::eval_predicate(row, schema, &predicate));
    // ...
}
```

`retain()` 遍历所有行，对于每行都要调用 `eval_predicate()`。当表中有 10,000 行时，每次 DELETE 都需要检查 10,000 行。

**问题2：列名查找开销（O(c) per row）**

`eval_predicate()` 中每次比较都需要在 schema 中查找列名位置：

```rust
fn eval_predicate(row: &[Value], schema: &Schema, predicate: &Predicate) -> bool {
    match predicate {
        Predicate::Eq(col_name, val) => {
            // 每次调用都执行 columns().iter().position() —— O(c)
            if let Some(idx) = schema.columns().iter().position(|c| c.name() == col_name) {
                &row[idx] == val
            } else { false }
        }
        // ...
    }
}
```

对于 `id` 列，每次比较都要遍历 schema 的列定义。虽然在本例中只有 3 列（O(3) 可忽略），但这是不必要的时间开销。

**问题3：Vec 删除的移动成本**

`Vec::retain()` 在删除元素时，需要将后面的元素向前移动（memcpy），对于 10,000 行规模的表，每次 DELETE 都需要移动平均约 5,000 个元素。

#### 2.3 根因分析汇总

| 可能原因 | 为什么影响 DELETE | 优化方向 |
|----------|------------------|----------|
| 1. 全表扫描 O(n) | 每次 DELETE 遍历全部 10,000 行，即使用 `retain()` 也如此 | 引入主键索引（HashMap），直接定位行 |
| 2. 列名查找开销 | 每次谓词评估都调用 `position()` 在 schema 中查找列名 | 预计算列索引，缓存到谓词结构中 |
| 3. Vec 删除移动成本 | `retain()` 删除元素后需要 memcpy 移动后续元素 | 使用标记删除（tombstone）或交换删除 |
| 4. UPDATE 的三步开销 | UPDATE = read + delete + write，每个操作都独立扫描 | 实现真正的 UPDATE 操作，一次扫描完成 |

> 📝 AI 分析记录：`reports/week-10/screenshots/root_cause_analysis.txt`

---

### 步骤3：模拟 Gate 检查

#### 3.1 BP1 静态检查

```bash
echo "=== BP1 静态检查 ==="
cargo build --all-features    # 编译检查
cargo fmt --check --all       # 格式检查
cargo clippy --all-features -- -D warnings  # Lint 检查
cargo test --lib              # 单元测试
```

**BP1 检查结果**：

| BP1 检查项 | 结果 |
|-----------|------|
| 编译 (cargo build) | ✅ 通过 |
| 格式 (cargo fmt) | ✅ 通过 |
| Clippy (cargo clippy) | ✅ 通过 |
| 单元测试 (cargo test --lib) | ✅ 通过 (126 passed) |

**BP1 结论**：✅ **ALL PASS** — 代码通过所有静态检查。

> 📋 BP1 输出：`reports/week-10/screenshots/gate_bp1_output.txt`

#### 3.2 BP2 行为检查

```bash
echo "=== BP2 行为检查 ==="
cargo test --test qps_benchmark_test -- --ignored --nocapture
```

**BP2 检查结果**：

| BP2 检查项 | 实测 QPS | E-09 地板 | 是否通过 |
|-----------|---------|----------|---------|
| DELETE QPS | 5,105 | ≥ 10,000 | ❌ **FAIL** |
| UPDATE QPS | 1,384 | ≥ 10,000 | ❌ **FAIL** |

**BP2 结论**：❌ **BLOCKED** — DELETE 和 UPDATE 均未达到 E-09 硬性地板，PR 被拦截。

> 📋 BP2 输出：`reports/week-10/screenshots/gate_bp2_output.txt`

#### 3.3 理解"被 Gate 拦截"

```
DELETE QPS = 5,105 < 10,000 (E-09地板)
  → BP2 Gate: FAIL
  → PR #500 被 Blocked
  → 开发者收到通知："DELETE QPS 未达到 E-09 地板 (10,000)"
  → 必须修复后重新提交

这不是"你的代码不好"，
而是"客观指标告诉你：这个版本还不能发布"。
```

**类比**：体检报告说你血压偏高——不是医生在骂你，是数据在提醒你。Gate 系统把"主观判断"（"我觉得够快了"）替换为"客观指标"（"QPS ≥ 10,000 才算达标"）。

---

### 步骤4：Harness 治理闭环图

```
┌──────────────────────────────────────────────────────────────────────┐
│                    Harness 治理闭环 —— DELETE/UPDATE 优化案例         │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ① Issue: "DELETE QPS 需从 5,105 → ≥10,000"                          │
│           "UPDATE QPS 需从 1,384 → ≥10,000"                           │
│          │                                                            │
│          ▼                                                            │
│  ② AI/人 分析根因 → 提出优化方案 → 实现代码                           │
│     · 方案A: 引入 HashMap 主键索引 (id → row_index)                   │
│     · 方案B: 预计算列索引，缓存到 Predicate                           │
│     · 方案C: 批量 DELETE，减少 retain() 调用次数                       │
│     · 方案D: 实现 swap_remove 替代 retain                              │
│          │                                                            │
│          ▼                                                            │
│  ③ Gate 检查                                                          │
│     BP1 (静态): ✅ build ✅ fmt ✅ clippy ✅ test                       │
│     BP2 (行为): DELETE=5,105 ❌ (<10,000)  UPDATE=1,384 ❌ (<10,000)   │
│          │                                                            │
│          │  ❌ 拦截! 回到步骤②                                        │
│          │                                                            │
│          ▼ (经过多轮优化后...)                                        │
│  ④ Gate 重新检查                                                      │
│     BP1: ✅ 全部通过                                                   │
│     BP2: DELETE=XX,XXX ✅  UPDATE=XX,XXX ✅                            │
│          │                                                            │
│          ▼                                                            │
│  ⑤ PR 合并到 develop/v3.0.0                                           │
│          │                                                            │
│          ▼                                                            │
│  ⑥ 知识沉淀                                                           │
│     · 更新基准数据（QPS 从 5,105/1,384 提升至目标值）                  │
│     · 记录优化 Pattern：主键索引、列索引缓存、批量操作                  │
│     · 评估：当前 10,000 的 Gate 阈值是否需要根据新数据调整？             │
│          │                                                            │
│          └──→ 回到 ①（下一轮性能优化：SELECT 的 3,294 QPS）            │
│                                                                       │
│  ⚠️ 如果 Gate 不存在：                                                 │
│     DELETE 5,105 QPS 的版本会被直接发布，                              │
│     生产环境清空 10,000 行表需要 2 秒（而非 <1 秒），                   │
│     用户感知到明显卡顿，但问题直到用户投诉才会被发现。                    │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

---

### 步骤5：三层模型反思

#### 5.1 对照 DELETE/UPDATE 案例

```
┌─────────────────────────────────────────────────────────────────────┐
│                      Harness 三层治理模型                             │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  提示词层 (Prompt Layer)                                              │
│  ─────────────────────────                                           │
│  · 如果只说"帮我优化 DELETE 性能"                                     │
│    → AI 可能只是做微小调整（如 inline 化函数），达不到 10x 提升        │
│  · 正确做法：PR 模板必须写清楚量化目标                                 │
│    "将 DELETE QPS 从 5,105 提升至 ≥10,000，                                 │
│     方案需包含主键索引优化"                                           │
│                                                                      │
│  上下文层 (Context Layer)                                             │
│  ─────────────────────────                                           │
│  · AI 能看到：当前 DELETE 的实现代码 (memory.rs)                       │
│  · AI 看不到：这个 DELETE 在 10,000 行规模下的实际表现                 │
│               不同 Vec 删除策略的 CPU 缓存行为                        │
│               真实生产环境中并发 DELETE 的竞争情况                    │
│  · 治理改进：向 AI 提供基准测试数据 + 性能 Profile                     │
│                                                                      │
│  Harness 层 (Gate Layer)                                              │
│  ───────────────────────                                             │
│  · Gate 不管"AI 怎么实现的"，只管"QPS 达标了吗？"                      │
│  · BP2: DELETE QPS 实测 → ≥10,000 ✅ / <10,000 ❌                      │
│  · Harness = 把"主观判断"变成"客观指标"                               │
│  · 同理适用于：测试覆盖率、代码风格、编译警告                          │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

#### 5.2 个人反思

| 层级 | 本次实验中注意到的问题 | 下次如何改进 |
|------|----------------------|------------|
| **提示词层** | 初次写基准测试时，直接用 `engine.data` (私有字段) 导致编译失败——我没有给 AI 足够的上下文说明字段可见性 | 在提示词中明确说明 "MemoryStorage 的 data 字段是私有的，请只用公开 API" |
| **上下文层** | AI 分析根因时只看到了代码结构，不知道实际 QPS 数据。我补充数据后 AI 的分析才变得具体 | 在 PR 描述中附上基准测试数据，让 AI 和审查者都有完整上下文 |
| **Harness 层** | 有了 Gate 的 QPS 门槛，优化就有了明确的目标——不是"尽量快"，而是"必须达到 10,000"。Gate 让模糊的需求变成了可度量的约束 | 对所有关键操作设定定量性能基线，作为 CI 的一部分 |

---

## 五、实验总结

### 5.1 知识收获

1. **性能基准的重要性**：没有基准数据，就无法判断"慢不慢"、"优化了没有"、"是否达标"。INSERT 的 120 万 QPS 和 UPDATE 的 1,384 QPS 之间的 877 倍差距，只有通过基准测试才能发现。
2. **根因分析的方法论**：从"哪个操作慢"→"慢操作做了什么"→"哪些步骤是不必要的"→"如何消除不必要步骤"，这是系统性的性能分析方法。
3. **Gate 系统的设计哲学**：Gate 不判断代码好坏，只判断指标是否达标。这是一种"以结果为导向"的治理模式，适用于 AI 生成代码的质量控制。
4. **三层模型的实际意义**：提示词层管"目标是否清晰"，上下文层管"信息是否充分"，Harness 层管"结果是否达标"。三层各司其职，缺一不可。

### 5.2 心得体会

- **Harness 不是"帮 AI 写代码"，而是"告诉 AI 哪里还不够好"**：本实验中，AI 可以写代码，但它不知道 QPS 门槛是 10,000。Harness 通过 Gate 系统告诉开发者（和 AI）：当前版本还不够好，请继续优化。
- **Benchmark 是 Harness 的眼睛**：没有基准测试的 Harness 是盲目的——它不知道代码是否满足性能要求。基准测试是 Harness 治理闭环中"测量"环节的核心。
- **约束驱动优化**：有了明确的 QPS 门槛，优化就从"尽量做"变成了"必须做"。这种约束驱动的开发模式更适合 AI 辅助开发——AI 擅长在有明确目标的情况下迭代优化。

### 5.3 改进建议

1. **将 QPS 基准测试加入 CI 流水线**：每次 PR 自动运行基准测试，与 E-09 地板比较
2. **性能 Profile 应作为上下文提供给 AI**：在 PR 描述中附上 perf/flamegraph 数据，帮助 AI 更准确地定位瓶颈
3. **Gate 阈值应该定期校准**：如果硬件升级（更快的 CPU），10,000 QPS 的门槛可能需要上调
4. **不同操作应有不同权重**：DELETE 比 INSERT 慢是正常的，E-09 地板应该区分操作类型

---

## 六、AI 工具使用记录

| AI 工具 | 使用场景 | 效果评价 |
|--------|----------|----------|
| Claude Code | 根因分析：为什么 DELETE 比 INSERT 慢 237 倍？ | 从 Vec 数据结构、retain 算法复杂度、schema 查找开销三个角度分析，分析全面 |
| Claude Code | 基准测试代码编写辅助 | 帮助设计了正确的 benchmark 结构（ignore + nocapture） |
| Claude Code | Gate 模拟结果解读 | 解释了 BP1/BP2 在治理中的不同角色 |

---

## 七、参考资料

1. Harness 治理框架设计文档
2. Rust Vec::retain 源码与性能分析, https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain
3. SQLRustGo 项目架构文档, `docs/architecture.md`
4. Criterion.rs Benchmarking Guide, https://bheisler.github.io/criterion.rs/book/

---

## 八、教师评语

| 评分项 | 分值 | 得分 |
|--------|------|------|
| QPS 基准测试数据（4组） | 15 | |
| 根因分析（AI辅助 + 人工判断） | 20 | |
| Gate 模拟结果 | 20 | |
| Harness 治理闭环图 | 25 | |
| 三层模型反思 | 10 | |
| 实验报告完整 | 10 | |
| **总分** | **100** | |

教师签名：__________ 日期：__________

---

## 附录

### 附录 A：性能数据汇总

| 操作 | QPS | 排名 | 相对 INSERT 倍数 |
|------|-----|------|-----------------|
| INSERT | 1,213,209 | 🥇 最快 | 1x（基准） |
| DELETE | 5,105 | 🥈 | 慢 237x |
| SELECT | 3,294 | 🥉 | 慢 368x |
| UPDATE | 1,384 | 🐢 最慢 | 慢 877x |

### 附录 B：文件清单

| 文件 | 路径 | 说明 |
|------|------|------|
| 实验报告 | `reports/week-10/README.md` | 本文件 |
| 基准测试源码 | `tests/qps_benchmark_test.rs` | QPS 基准测试 |
| 基准测试输出 | `reports/week-10/screenshots/benchmark_output.txt` | 四组测试结果 |
| BP1 检查结果 | `reports/week-10/screenshots/gate_bp1_output.txt` | 静态检查 |
| BP2 检查结果 | `reports/week-10/screenshots/gate_bp2_output.txt` | 行为检查 |

---

*最后更新: 2026-06-06*
*Harness 治理实战第1周 — DELETE/UPDATE 性能优化案例*
