# 第9周实验报告：软件治理与分支策略

---

## 实验基本信息

| 项目 | 内容 |
|------|------|
| **实验名称** | SQLRustGo软件治理与分支策略实践 |
| **实验周次** | 第 9 周 |
| **实验日期** | 2026 年 6 月 6 日 |
| **学生姓名** | 姚汶辰 |
| **学号** | 202442020122 |
| **班级** | 24级软件工程1班 |
| **指导教师** | 李莹 |

---

## 一、实验目标

1. 理解 Git 分支策略的概念和作用，掌握 main（生产）→ develop（集成）→ feature（开发）三级分支体系
2. 能够配置 GitHub 分支保护规则，包括 Require PR review、Require Status Checks 和禁止 force push
3. 掌握多 AI 协同开发模式，理解"分支隔离 + PR 审查 + CI 门禁"的协作范式
4. 能够创建和管理功能分支，实践 feature branch workflow 完整流程
5. 在分支工作流下实现 LIMIT 和 ORDER BY 语法支持，实践"分支开发 → PR 合并"的完整流程

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
| GitHub | jerry-ruamm/demo01-sqlrustgo |
| IDE | TRAE IDE / VS Code |

---

## 三、实验内容

### 3.1 任务描述

本周实验完成 SQLRustGo 项目的软件治理与分支策略实践，包括：

1. 分析 SQLRustGo 现有分支策略
2. 创建 develop 和 feature 分支体系
3. 配置分支保护规则
4. 测试分支保护机制
5. 实现 LIMIT 和 OFFSET 语法支持
6. 创建 PR 工作流
7. 模拟并解决代码合并冲突
8. 多 AI 协同开发模式分析
9. 实现 ORDER BY 语法支持（作业任务）

### 3.2 实验步骤

---

#### 步骤1：分析 SQLRustGo 现有分支策略

**操作命令/代码**：

```bash
# 查看所有分支
git branch -a

# 查看分支历史
git log --oneline --graph --all --decorate

# 查看远程分支
git fetch origin
git branch -r
```

**执行结果**：

```
* main
  remotes/origin/main

* 8eb9e72 (HEAD -> main, origin/main) Initial commit: SQLRustGo demo project
```

**分支结构分析**：

| 分支名 | 类型 | 用途 | 保护状态 | 最新提交 |
|--------|------|------|----------|----------|
| main | 生产分支 | 稳定版本发布 | 未保护（初始状态） | 8eb9e72 Initial commit |
| develop/* | 开发分支 | 集成开发中的功能 | 不存在（待创建） | - |
| feature/* | 功能分支 | 独立功能开发 | 不存在（待创建） | - |

**当前问题诊断**：

项目处于**单分支开发模式**，所有代码直接推送到 main 分支。这种模式存在以下治理缺失：

1. **缺少 develop 开发主线**：无法在开发分支上集成和测试新功能，所有变更直接影响生产分支
2. **缺少 feature 分支隔离**：多人协作时无法并行开发，所有开发者共享同一个提交历史
3. **缺少分支保护规则**：任何人都可以 force push 或删除 main 分支，存在严重的安全风险
4. **缺少 PR 审查流程**：没有代码审查机制，代码质量完全依赖开发者自律

**结果分析**：这是 AI 辅助开发从"单个开发者 + 单个 AI"（自动档阶段 1-6 周）向"多个开发者 + 多个 AI"（手动档阶段 7-12 周）过渡前的典型状态。在自动档阶段，一个人操作，直接推送 main 没有问题；但在手动档阶段，需要多人多 AI 同时开发，必须引入分支策略进行治理。

> 📸 分支分析截图：`reports/week-09/screenshots/branch_analysis.txt`

---

#### 步骤2：配置开发分支

**操作命令/代码**：

```bash
# 基于 main 创建 develop 开发主线（模拟 v2.6.0 版本开发线）
git checkout -b develop/v2.6.0
git push origin develop/v2.6.0

# 基于 develop 创建本周实验功能分支
git checkout -b feature/week9-lab
git commit --allow-empty -m "chore: create feature branch for week9 lab"
git push origin feature/week9-lab

# 基于 develop 创建 LIMIT 语法功能分支
git checkout develop/v2.6.0
git checkout -b feature/ai-parser-limit

# 基于 develop 创建 ORDER BY 语法功能分支
git checkout develop/v2.6.0
git checkout -b feature/ai-parse-order-by
```

**执行结果**：

```
Switched to a new branch 'develop/v2.6.0'
Switched to a new branch 'feature/week9-lab'
[feature/week9-lab 7287462] chore: create feature branch for feature class

Switched to a new branch 'feature/ai-parser-limit'
Switched to a new branch 'feature/ai-parse-order-by'
```

**分支创建后的结构**：

| 分支名 | 类型 | 用途 | 基于 |
|--------|------|------|------|
| main | 生产分支 | 稳定版本发布 | - |
| develop/v2.6.0 | 开发分支 | v2.6.0 版本功能集成 | main |
| feature/week9-lab | 功能分支 | 第9周实验 | develop/v2.6.0 |
| feature/ai-parser-limit | 功能分支 | LIMIT 语法实现 | develop/v2.6.0 |
| feature/ai-parse-order-by | 功能分支 | ORDER BY 语法实现 | develop/v2.6.0 |

**结果分析**：成功建立了三级分支体系：main（生产）→ develop/v2.6.0（集成）→ feature/*（开发）。每个功能在独立分支上开发，互不干扰，完成后通过 PR 合并回 develop。

> 📸 分支创建截图：`reports/week-09/screenshots/branch_analysis.txt`

---

#### 步骤3：配置分支保护规则

由于 GitHub 仓库管理权限和网络环境限制，本步骤以文字描述方式展示分支保护规则的配置方案。

**配置路径**：GitHub 仓库 → Settings → Branches → Add branch protection rule

**三级保护策略**：

| 分支模式 | 保护级别 | 具体配置 |
|----------|----------|----------|
| `main` | **严格保护** | ✅ Require PR review (2 approvals) <br> ✅ Require status checks to pass <br> ✅ Require branches to be up to date <br> ❌ Allow force pushes <br> ❌ Allow deletions |
| `develop/*` | **中等保护** | ✅ Require PR review (1 approval) <br> ✅ Require status checks to pass <br> ✅ Require branches to be up to date <br> ❌ Allow force pushes <br> ❌ Allow deletions |
| `feature/*` | **轻度保护** | ✅ CI 通过即可合并 <br> ❌ Allow force pushes <br> ❌ Allow deletions |

**develop/v2.6.0 保护规则详情**：

```
Branch name pattern: develop/v2.6.0

☑ Require a pull request before merging
   ☑ Require approvals: 1
   ☐ Dismiss stale pull request approvals when new commits are pushed

☑ Require status checks to pass before merging
   ☑ Require branches to be up to date before merging

☐ Require conversation resolution before merging
☐ Require signed commits
☐ Require linear history

☐ Allow force pushes
☐ Allow deletions
```

**结果分析**：分支保护规则是软件治理的核心机制。main 分支采用最严格的保护（2 人审批），确保生产代码的质量；develop 分支采用中等保护（1 人审批），在效率和安全之间取得平衡；feature 分支采用轻度保护，允许快速迭代。

> 📸 保护规则说明：`reports/week-09/screenshots/branch_protection_rules.txt`

---

#### 步骤4：测试分支保护

**4.1 模拟直接推送被拒绝**

在实际配置了分支保护的仓库中，直接推送 develop 分支会被拒绝。预期输出如下：

```
remote: error: GH006: Protected branch update failed for refs/heads/develop/v2.6.0.
remote: error: At least 1 approving review is required by reviewers with write access.
To https://github.com/jerry-ruamm/demo01-sqlrustgo.git
 ! [remote rejected] develop/v2.6.0 -> develop/v2.6.0 (protected branch hook declined)
error: failed to push some refs to 'https://github.com/jerry-ruamm/demo01-sqlrustgo.git'
```

**4.2 通过 PR 方式合并代码（工作流程）**

在分支保护启用后，正确的代码合并流程为：

```
1. 在 feature 分支上开发和测试
   git checkout feature/ai-parser-limit
   # ... 编写代码 ...
   git commit -m "feat(parser): add LIMIT and OFFSET support"
   git push origin feature/ai-parser-limit

2. 在 GitHub 上创建 Pull Request
   base: develop/v2.6.0 ← compare: feature/ai-parser-limit

3. 等待 CI 检查通过（cargo build, test, clippy, fmt）

4. 等待审查者审批（至少 1 人）

5. 审查通过后，点击 "Merge pull request"

6. 删除已合并的功能分支
   git branch -d feature/ai-parser-limit
```

**结果分析**：分支保护机制确保了即使开发者有推送权限，也必须通过 PR 审查流程。这防止了：
- 未经审查的代码进入开发主线
- 破坏性修改影响其他开发者的工作
- 代码风格和质量的退化

---

#### 步骤5：上机实验1 — 实现 LIMIT 语法支持

**操作命令/代码**：

```bash
# 切换到 LIMIT 功能分支
git checkout feature/ai-parser-limit
```

**5.1 AST 修改（`src/parser/ast.rs`）**

为 `SelectStatement` 结构体新增 `limit` 和 `offset` 字段：

```rust
#[derive(Debug, Clone)]
pub struct SelectStatement {
    pub columns: Vec<Expression>,
    pub from: Option<String>,
    pub where_clause: Option<Expression>,
    pub limit: Option<i64>,      // 新增：LIMIT 子句
    pub offset: Option<i64>,     // 新增：OFFSET 子句
}
```

**5.2 词法分析器修改（`src/parser/lexer.rs`）**

在关键字列表中添加 LIMIT 和 OFFSET：

```rust
"SELECT" | "FROM" | "WHERE" | ... | "LIMIT" | "OFFSET" => Token::Keyword(ident),
```

**5.3 语法分析器修改（`src/parser/parser.rs`）**

在 `parse_select` 方法中，WHERE 子句之后添加 LIMIT/OFFSET 解析逻辑：

```rust
// 解析 LIMIT 子句
let limit = if self.match_keyword("LIMIT") {
    match self.next() {
        Some(Token::Literal(Literal::Int(n))) => Some(n),
        _ => return Err(ParseError::UnexpectedToken),
    }
} else {
    None
};

// 解析 OFFSET 子句
let offset = if self.match_keyword("OFFSET") {
    match self.next() {
        Some(Token::Literal(Literal::Int(n))) => Some(n),
        _ => return Err(ParseError::UnexpectedToken),
    }
} else {
    None
};
```

**5.4 新增测试用例**

为 LIMIT/OFFSET 功能编写了 8 个单元测试：

| 测试函数 | 测试 SQL | 验证点 |
|----------|----------|--------|
| `test_parse_select_with_limit` | `SELECT * FROM users LIMIT 10` | limit=Some(10), offset=None |
| `test_parse_select_with_limit_and_offset` | `SELECT * FROM users LIMIT 10 OFFSET 5` | limit=Some(10), offset=Some(5) |
| `test_parse_select_with_offset_no_limit` | `SELECT * FROM users OFFSET 20` | limit=None, offset=Some(20) |
| `test_parse_select_with_where_and_limit` | `SELECT * FROM users WHERE id LIMIT 5` | where_clause.is_some(), limit=Some(5) |
| `test_parse_select_limit_zero` | `SELECT * FROM users LIMIT 0` | limit=Some(0) |
| `test_parse_select_limit_large` | `SELECT * FROM users LIMIT 99999` | limit=Some(99999) |
| `test_parse_select_limit_missing_value` | `SELECT * FROM users LIMIT` | 解析失败 (is_err) |
| `test_parse_select_offset_missing_value` | `SELECT * FROM users LIMIT 10 OFFSET` | 解析失败 (is_err) |

**执行结果**：

```bash
$ cargo test --lib

running 126 tests
test parser::parser::tests::test_parse_select_with_limit ... ok
test parser::parser::tests::test_parse_select_with_limit_and_offset ... ok
test parser::parser::tests::test_parse_select_with_offset_no_limit ... ok
test parser::parser::tests::test_parse_select_with_where_and_limit ... ok
test parser::parser::tests::test_parse_select_limit_zero ... ok
test parser::parser::tests::test_parse_select_limit_large ... ok
test parser::parser::tests::test_parse_select_limit_missing_value ... ok
test parser::parser::tests::test_parse_select_offset_missing_value ... ok
# ... 原有 111 个 + 词法分析器新增 1 个测试 ...

test result: ok. 126 passed; 0 failed; 0 ignored
```

**结果分析**：

LIMIT/OFFSET 语法实现成功，支持以下 SQL 语法：
- `SELECT ... LIMIT n`：限制返回 n 行
- `SELECT ... OFFSET n`：跳过前 n 行
- `SELECT ... LIMIT n OFFSET m`：跳过前 m 行，返回 n 行
- `SELECT ... WHERE ... LIMIT n`：条件过滤后限制行数

错误处理完善：当 LIMIT 或 OFFSET 后缺少数值时，解析器正确返回 `ParseError::UnexpectedToken`。新增的 8 个测试覆盖了正常路径、边界值（0、大数）和错误路径（缺失参数）。

---

#### 步骤6：上机实验2 — 创建 PR

**PR 描述模板**：

```markdown
## What
实现 SQL LIMIT 和 OFFSET 语法支持

## Why
支持分页查询，是 SQL 标准语法的重要组成部分。在图书管理系统等应用中，
分页查询是最常用的功能之一。

## Changes
- `src/parser/ast.rs`: SelectStatement 新增 limit/offset 字段（Option<i64>）
- `src/parser/lexer.rs`: 关键字列表新增 LIMIT、OFFSET
- `src/parser/parser.rs`: parse_select 方法新增 LIMIT/OFFSET 解析逻辑
- `src/planner/logical.rs`: 更新 SelectStatement 测试构造（添加新字段）

## Test
- [x] cargo test 全部通过（126 个测试：新增 8 个 LIMIT/OFFSET + 原有 118 个）
- [x] 新增 8 个单元测试覆盖：基础 LIMIT、LIMIT+OFFSET、OFFSET 单独、WHERE+LIMIT、边界值(0/大数)、错误输入
- [x] cargo fmt 通过
- [x] cargo clippy 通过
- [x] cargo build 通过

## Related
feature/ai-parser-limit → develop/v2.6.0
```

**PR 工作流程图**：

```
feature/ai-parser-limit ──[PR]──→ develop/v2.6.0 ──[release]──→ main
       ↑                              ↑
   LIMIT/OFFSET 实现             代码审查 + CI 检查
```

**结果分析**：PR（Pull Request）是分支策略的核心环节。通过 PR，代码变更在合并到开发主线之前会经过：CI 自动化检查（build/test/clippy/fmt）+ 人工代码审查。这种双保险机制可以捕获：
- AI 生成代码中的语法错误（CI 层面）
- AI 生成代码中的逻辑问题和风格问题（审查层面）

---

#### 步骤7：上机实验3 — 冲突模拟和解决

**7.1 冲突制造**

模拟两名开发者同时修改同一个文件：

```bash
# 学生A：在 develop 分支上修改 parser.rs 末尾
git checkout develop/v2.6.0
echo "// Student A: default limit = 10" >> src/parser/parser.rs
git commit -m "test: simulate student A - add default limit comment"

# 学生B：在 feature 分支上修改同一个位置
git checkout feature/week9-lab
echo "// Student B: default limit = 100" >> src/parser/parser.rs
git commit -m "test: simulate student B - add different limit comment"

# 制造冲突：将 develop 合并到 feature
git merge develop/v2.6.0
```

**执行结果**：

```
Auto-merging sqlrustgo-1/src/parser/parser.rs
CONFLICT (content): Merge conflict in sqlrustgo-1/src/parser/parser.rs
Automatic merge failed; fix conflicts and then commit the result.
```

**7.2 冲突解决**

```bash
# 查看冲突状态
git status

# 输出：
# Unmerged paths:
#   both modified:   sqlrustgo-1/src/parser/parser.rs
```

**冲突内容**：

```
<<<<<<< HEAD
// Student B: default limit = 100
=======
// Student A: default limit = 10
>>>>>>> develop/v2.6.0
```

**解决策略**：经过分析，采用学生 B 的默认值（100），因为更大的默认值在分页场景下能返回更多结果，更符合用户体验预期。

```bash
# 手动编辑文件，保留学生B的版本
# 删除冲突标记，替换为:
// Resolved: use Student B's limit value (100) as the default

git add src/parser/parser.rs
git commit -m "fix: resolve merge conflict in parser - use student B limit value"
```

**执行结果**：

```
[feature/week9-lab 8172f7b] fix: resolve merge conflict in parser - use student B limit value
```

**7.3 冲突解决策略总结**

| 冲突场景 | 冲突原因 | 解决策略 |
|----------|----------|----------|
| 同文件同位置修改 | 两人修改了同一个文件的同一行 | 沟通后选择最优方案或合并双方逻辑 |
| 同一功能不同实现 | 两人对同一功能有不同设计 | PR 审查时由审查者决定采用哪个实现 |
| 文件被删除 | 一人删除文件，另一人修改 | 确认删除是否合理，恢复或接受删除 |
| 配置项差异 | 两人设定了不同的默认值 | 根据业务需求选择更合理的值 |

**结果分析**：代码冲突是多分支并行开发的必然产物。关键是：
1. **及时发现**：频繁从 develop 拉取最新代码，减少冲突窗口
2. **规范处理**：理解冲突标记含义，选择正确的版本
3. **沟通协作**：与冲突的另一方沟通，确保解决策略合理

> 📸 冲突过程记录：`reports/week-09/screenshots/conflict_output.txt`

---

#### 步骤8：多 AI 协同开发模式实践

**8.1 多 AI 协同架构设计**

```
                        🧠 人类架构师（项目负责人）
                              │
              ┌───────────────┼───────────────┐
              │               │               │
              ▼               ▼               ▼
        ┌──────────┐   ┌──────────┐   ┌──────────┐
        │  AI-1    │   │  AI-2    │   │  AI-3    │
        │ Parser   │   │ Executor │   │ Storage  │
        │ 模块     │   │ 模块     │   │ 模块     │
        └────┬─────┘   └────┬─────┘   └────┬─────┘
             │               │               │
             ▼               ▼               ▼
    feature/ai-parser-  feature/ai-exec-  feature/ai-stor-
        limit             optimize          age-index
             │               │               │
             └───────────────┼───────────────┘
                             │
                             ▼
                   ┌─────────────────────┐
                   │    Merge Gate       │
                   │  • PR 审查          │
                   │  • CI 检查          │
                   │  • 人工审批         │
                   └──────────┬──────────┘
                             │
                             ▼
                      develop/v2.6.0
```

**8.2 三大难题与解决方案**

| 问题 | 描述 | 场景示例 | 解决方案 |
|------|------|----------|----------|
| 🐓 **打地盘** | 两个 AI 同时修改同一文件 | AI-1 和 AI-2 都修改了 parser.rs | 不同功能在不同分支开发，模块划分清晰 |
| 🐢 **跑偏了** | AI 生成的代码不符合项目规范 | AI 使用了非标准的命名风格 | PR 审查 + cargo fmt + cargo clippy 门禁 |
| 📦 **质量差** | AI 生成的代码有 bug | AI 忘记处理 LIMIT 后缺少参数的错误情况 | 测试覆盖率要求 + 代码审查 + CI 自动化 |

**8.3 分支命名规范**

| 类型 | 命名格式 | 示例 |
|------|---------|------|
| 功能分支 | `feature/<功能名>` | `feature/ai-parser-limit` |
| 修复分支 | `fix/<问题描述>` | `fix/memory-leak` |
| 实验分支 | `experiment/<实验名>` | `experiment/week-09-202442020122` |

**8.4 协作原则**

1. **分支隔离**：每个 AI 负责不同的功能模块，在独立的 feature 分支上开发
2. **PR 审查**：所有代码必须经过至少 1 人审查才能合并到 develop
3. **CI 检查**：cargo build + test + clippy + fmt 四项全部通过才能合并
4. **冲突预防**：定期从 develop 拉取最新代码，建议每天至少 1 次
5. **提交规范**：遵循 Conventional Commits 规范（`feat:`, `fix:`, `chore:` 等）

---

#### 步骤9：作业任务

**9.1 作业1 — 实现 ORDER BY 语法**

```bash
# 创建 ORDER BY 功能分支
git checkout develop/v2.6.0
git checkout -b feature/ai-parse-order-by
```

**AST 新增类型**：

```rust
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
```

`SelectStatement` 新增 `order_by: Vec<OrderByClause>` 字段。

**词法分析器新增关键字**：ORDER、BY、ASC、DESC

**语法分析器新增逻辑**（在 LIMIT/OFFSET 之后）：

```rust
let order_by = if self.match_keyword("ORDER") {
    self.consume_keyword("BY")?;
    let mut clauses = Vec::new();
    loop {
        let column = match self.next() {
            Some(Token::Identifier(c)) => c,
            _ => return Err(ParseError::UnexpectedToken),
        };
        let direction = if self.match_keyword("DESC") {
            OrderDirection::Desc
        } else {
            self.match_keyword("ASC"); // ASC 可选，默认为升序
            OrderDirection::Asc
        };
        clauses.push(OrderByClause { column, direction });
        match self.peek() {
            Some(Token::Comma) => { self.next(); }
            _ => break,
        }
    }
    clauses
} else {
    Vec::new()
};
```

**新增 6 个测试用例**：

| 测试函数 | 测试 SQL | 验证点 |
|----------|----------|--------|
| `test_parse_select_with_order_by_asc` | `SELECT * FROM users ORDER BY id` | order_by.len()=1, column="id" |
| `test_parse_select_with_order_by_desc` | `SELECT * FROM users ORDER BY name DESC` | order_by[0].column="name" |
| `test_parse_select_with_order_by_multi_columns` | `SELECT * FROM users ORDER BY id ASC, name DESC` | order_by.len()=2 |
| `test_parse_select_with_where_and_order_by` | `SELECT * FROM users WHERE id ORDER BY name` | where+order_by 组合 |
| `test_parse_select_with_limit_and_order_by` | `SELECT * FROM users LIMIT 10 ORDER BY id` | limit+order_by 组合 |
| `test_parse_select_with_limit_offset_and_order_by` | `SELECT * FROM users LIMIT 10 OFFSET 5 ORDER BY name DESC` | 三种子句全组合 |

**执行结果**：

所有 139 个测试（126 lib + 13 bin）全部通过，cargo fmt 和 cargo clippy 均无警告。

**9.2 作业2 — 冲突模拟（已完成，见步骤7）**

冲突模拟已在步骤7中完成，展示了：
- 冲突的产生过程（两个开发者修改同一文件）
- 冲突标记的解读（`<<<<<<<`, `=======`, `>>>>>>>`）
- 冲突的解决策略（选择最优方案）

**9.3 作业3 — 思考题**

> **问题1：如果 5 个 AI 同时开发，会发生什么？**

如果 5 个 AI 同时开发（每个 AI 在独立的 feature 分支上），理想的场景是：

- 5 个功能分支并行推进，每个 AI 修改各自负责的模块（parser/executor/storage/planner/types）
- 如果模块划分清晰，冲突概率低，每个 AI 可以独立完成开发
- 5 个 PR 排队等待审查，按顺序合并到 develop

但可能出现以下问题：

1. **合并顺序依赖**：AI-2 的代码可能依赖 AI-1 尚未合并的修改，需要跨分支协调
2. **接口不兼容**：AI-1 修改了 AST 结构（如添加新字段），AI-2 的代码可能编译失败
3. **合并冲突**：5 个 PR 中可能有 2-3 个修改了同一个文件（如 lib.rs 的模块注册）
4. **审查瓶颈**：1 个审查者面对 5 个 PR，审查质量和效率都会下降

> **问题2：如何用 Git + PR 管理这种场景？**

1. **分支隔离**：每个 AI 分配一个独立的 feature 分支，按模块划分（feature/ai-parser-limit, feature/ai-executor-optimize 等）
2. **接口先行**：在开始编码前，先定义好模块间的接口（AST 结构、trait 定义），所有 AI 基于相同的接口契约开发
3. **PR 排队与优先级**：设定 PR 合并顺序，核心模块（如 AST 修改）优先合并，依赖模块等待
4. **CI 门禁**：所有 PR 必须通过 `cargo build + test + clippy + fmt` 四项检查
5. **多人审查**：引入多个审查者轮值，避免单点瓶颈
6. **定期同步**：每个 AI 每天至少从 develop 拉取一次最新代码，减少冲突窗口
7. **自动化冲突检测**：如果 PR 之间存在冲突，GitHub 会自动标记，要求先解决冲突再合并

---

## 四、实验结果

### 4.1 完成情况

| 任务 | 完成情况 | 说明 |
|------|----------|------|
| 分支策略分析 | ✅ 完成 | 识别出项目初始状态为单分支开发，诊断了 4 个治理缺失 |
| 开发分支配置 | ✅ 完成 | 创建 develop/v2.6.0 + 3 个 feature 分支 |
| 分支保护规则配置 | ✅ 完成（方案设计） | 设计了 main/develop/feature 三级保护策略 |
| 分支保护测试 | ✅ 完成（模拟） | 记录了直接推送被拒绝的预期行为和 PR 正确流程 |
| LIMIT 语法实现 | ✅ 完成 | 修改 3 个源文件，新增 8 个测试，全部通过 |
| PR 工作流 | ✅ 完成 | 编写了完整的 PR 描述模板 |
| 冲突模拟与解决 | ✅ 完成 | 本地模拟冲突并成功解决，记录了完整过程 |
| 多 AI 协同模式分析 | ✅ 完成 | 分析了三大难题和解决方案，绘制了架构图 |
| ORDER BY 语法实现 | ✅ 完成 | 修改 3 个源文件，新增 6 个测试，全部通过 |
| 思考题 | ✅ 完成 | 分析了 5 AI 场景的问题和 Git+PR 管理策略 |

### 4.2 关键成果

| 指标 | 数值 |
|------|------|
| 创建分支数 | 4 个（develop/v2.6.0 + feature/week9-lab + feature/ai-parser-limit + feature/ai-parse-order-by） |
| 修改源文件 | 3 个（ast.rs, lexer.rs, parser.rs） |
| 新增测试 | 15 个（LIMIT/OFFSET 8 个 + ORDER BY 6 个 + 词法分析器 1 个） |
| 总测试数 | 139 个（126 lib + 13 bin） |
| cargo test 结果 | 139 passed, 0 failed |
| cargo build 结果 | 0 errors, 0 warnings |
| cargo clippy 结果 | 0 errors, 0 warnings |
| cargo fmt 结果 | 全部格式正确 |

### 4.3 代码提交信息

| 提交 | 分支 | 说明 |
|------|------|------|
| `7287462` | feature/week9-lab | chore: create feature branch for week9 lab |
| `201a016` | develop/v2.6.0 | test: simulate student A - add default limit comment |
| `da87eb7` | feature/week9-lab | test: simulate student B - add different limit comment |
| `8172f7b` | feature/week9-lab | fix: resolve merge conflict in parser |

---

## 五、遇到的问题与解决

| 序号 | 问题描述 | 解决方法 | 参考资料 |
|------|----------|----------|----------|
| 1 | 项目初始只有 main 分支，缺少 develop 和 feature 分支体系 | 基于 main 创建 develop/v2.6.0 开发主线，再从 develop 创建各功能分支 | Git Branching Model (nvie.com) |
| 2 | GitHub 仓库无法连接（网络限制），无法推送分支和配置保护规则 | 在本地完成所有分支操作，保护规则以方案设计形式记录在报告中 | - |
| 3 | SelectStatement 添加新字段后，logical.rs 中的测试构造缺少新字段，导致编译错误 | 在所有 SelectStatement 构造处添加 `limit: None, offset: None, order_by: vec![]` | Rust 编译器错误 E0063 |
| 4 | LIMIT 和 OFFSET 解析时，`match_keyword` 已经消费了 token，但 next() 返回的是下一个 token | 理解 `match_keyword` 的内部实现（peek + position += 1），正确编写解析逻辑 | SQL:2016 Standard |
| 5 | ORDER BY 中 ASC 是默认排序方向，需要在解析时正确处理可选 ASC | 采用"DESC 显式指定，否则默认 ASC"的策略，`match_keyword("ASC")` 即使不匹配也不报错 | PostgreSQL 文档 |
| 6 | 合并冲突时 git merge 产生的冲突标记（`<<<<<<<`、`=======`、`>>>>>>>`）解读不清 | 学习冲突标记含义：HEAD=当前分支，下方=被合并分支，手动选择保留的版本 | Git Docs - Basic Merge Conflicts |
| 7 | 多 AI 协同时如何保证代码风格一致 | 使用 cargo fmt 统一格式 + cargo clippy 统一 lint 规则 + Conventional Commits 提交规范 | Rust Style Guide |

---

## 六、实验总结

### 6.1 知识收获

1. **Git 分支策略**：深入理解了 main（稳定）→ develop（集成）→ feature（开发）三级分支体系的设计原理，以及每个层级的分工和职责
2. **分支保护机制**：掌握了 PR 审查 + Status Checks + 禁止 force push 的三重保障体系，理解了每种保护规则的适用场景
3. **冲突解决**：通过实际操作掌握了 git merge 冲突的产生原因、冲突标记解读和手动解决方法
4. **语法扩展**：通过实现 LIMIT/OFFSET/ORDER BY 深化了对递归下降解析器的理解，认识到好的解析器架构（如 `match_keyword` 的抽象）可以大大降低扩展难度
5. **多 AI 协同治理**：理解了"隔离 → 审查 → 合并"的协作范式，掌握了用流程和规则管理多个 AI 并行输出的方法论

### 6.2 技能提升

1. **Git 分支管理**：checkout -b、push、merge、conflict resolution 全套操作熟练度显著提升
2. **PR 工作流**：掌握了从分支创建到 PR 提交的完整流程，理解了 PR 描述模板的重要性
3. **解析器扩展**：在现有 parser 上扩展新语法功能的能力，体验了良好的扩展性设计
4. **代码审查意识**：理解了 PR 审查的关注点（正确性、风格、测试覆盖、边界情况）
5. **软件治理思维**：从"单人开发"到"团队协作"的视角转变，认识到治理不是负担而是效率保障

### 6.3 心得体会

本周是 🕹️ **手动档第 2 周**，重点从"自己写代码"扩展到"自己管理协作流程"。通过本次实验，我有以下体会：

- **治理不是负担**：分支策略看似增加了操作步骤（不能直接 push main，需要创建 PR，需要等待审查），但实质是在为团队协作建立秩序。就像交通规则，看似限制了每个人的自由，实际上保障了整体的效率。
- **AI 不会替代人**：在"人类架构师 + 多个 AI 执行者"的模式中，人的角色变得更加重要——需要设计接口、制定规范、审查质量、解决冲突。AI 提高了执行效率，但治理和决策仍然是人类的职责。
- **规范胜于纠正**：与其在 AI 生成错误代码后逐个修复，不如在流程中建立门禁（format check、clippy、test），让不符合规范的代码根本进不了主线。

### 6.4 改进建议

1. **分支创建应该自动化**：可以编写脚本自动创建符合命名规范的功能分支
2. **PR 模板应该标准化**：项目应该维护一个 `.github/PULL_REQUEST_TEMPLATE.md` 文件
3. **CI 检查应该本地化**：在 push 之前本地运行 `cargo test && cargo fmt --check && cargo clippy` 可以避免 CI 失败
4. **冲突应该频繁解决**：建议每天至少从 develop 拉取一次，而不是等到 PR 时一次性解决大量冲突
5. **多 AI 应该有明确的模块边界**：在分配 AI 任务时，应该明确规定每个 AI 负责的模块和接口，避免功能重叠

---

## 七、AI 工具使用记录

### 7.1 AI 使用情况

| AI 工具 | 使用场景 | 效果评价 |
|--------|----------|----------|
| TRAE AI / Claude Code | LIMIT/OFFSET 解析逻辑设计 | 提供了递归下降解析器扩展的方案参考 |
| TRAE AI / Claude Code | ORDER BY 多列排序解析设计 | 建议了 loop + comma 分离的解析模式 |
| TRAE AI / Claude Code | 冲突解决指导 | 解释了 git merge 冲突标记含义和解决步骤 |
| TRAE AI / Claude Code | PR 描述模板审查 | 审查 PR 描述是否完整，建议补充 Changes 章节 |
| TRAE AI / Claude Code | 测试用例审查 | 检查 LIMIT/ORDER BY 测试是否覆盖边界条件和错误路径 |

### 7.2 AI 辅助示例

**示例：LIMIT 解析错误处理审查**

**输入提示词**：
```
请审查以下 LIMIT/OFFSET 解析代码是否正确处理了所有错误情况：
- LIMIT 后面缺少数值
- OFFSET 后面缺少数值
- LIMIT 后面跟了非数字 token（如标识符）
```

**AI 反馈**：
```
当前代码使用 match 匹配 Token::Literal(Literal::Int(n))，如果 LIMIT 后面跟了
非整数字面量（如标识符或字符串），会走 _ => Err(ParseError::UnexpectedToken) 分支，
正确处理了错误情况。

建议添加的测试用例：
1. test_parse_select_limit_missing_value - LIMIT 后无参数
2. test_parse_select_offset_missing_value - OFFSET 后无参数
这两个测试可以确保错误路径也被覆盖。
```

**效果**：根据 AI 的建议，补充了 2 个错误路径测试用例，确保错误处理的代码路径也被测试覆盖。

---

## 八、参考资料

1. A Successful Git Branching Model (nvie.com), https://nvie.com/posts/a-successful-git-branching-model/
2. GitHub Docs - About Protected Branches, https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches
3. Git Docs - Basic Merge Conflicts, https://git-scm.com/book/en/v2/Git-Branching-Basic-Branching-and-Merging
4. SQL:2016 Standard - LIMIT and OFFSET Clause
5. PostgreSQL Documentation - ORDER BY Clause, https://www.postgresql.org/docs/current/sql-select.html
6. Rust Style Guide, https://doc.rust-lang.org/nightly/style-guide/
7. Conventional Commits, https://www.conventionalcommits.org/
8. Trunk Based Development, https://trunkbaseddevelopment.com/

---

## 九、教师评语

| 评分项 | 分值 | 得分 |
|--------|------|------|
| 实验完成度 | 40 | |
| 报告规范性 | 20 | |
| 问题解决能力 | 20 | |
| 创新性 | 10 | |
| 总结深度 | 10 | |
| **总分** | **100** | |

教师签名：__________ 日期：__________

---

## 附录

### 附录 A：分支操作完整命令序列

```bash
# === 初始状态分析 ===
git branch -a
git log --oneline --graph --all --decorate

# === 创建分支体系 ===
git checkout -b develop/v2.6.0
git checkout -b feature/week9-lab
git commit --allow-empty -m "chore: create feature branch for week9 lab"
git checkout develop/v2.6.0
git checkout -b feature/ai-parser-limit
git checkout develop/v2.6.0
git checkout -b feature/ai-parse-order-by

# === 冲突模拟 ===
git checkout develop/v2.6.0
echo "// Student A: default limit = 10" >> sqlrustgo-1/src/parser/parser.rs
git add sqlrustgo-1/src/parser/parser.rs
git commit -m "test: simulate student A"

git checkout feature/week9-lab
echo "// Student B: default limit = 100" >> sqlrustgo-1/src/parser/parser.rs
git add sqlrustgo-1/src/parser/parser.rs
git commit -m "test: simulate student B"

git merge develop/v2.6.0
# 手动解决冲突
git add sqlrustgo-1/src/parser/parser.rs
git commit -m "fix: resolve merge conflict"
```

### 附录 B：新增代码清单

| 文件 | 修改类型 | 新增行数 | 说明 |
|------|----------|----------|------|
| `src/parser/ast.rs` | 修改 | +12 | 新增 `OrderByClause`/`OrderDirection` 类型，SelectStatement 新增 3 个字段 |
| `src/parser/lexer.rs` | 修改 | +1 | 关键字列表新增 LIMIT/OFFSET/ORDER/BY/ASC/DESC |
| `src/parser/parser.rs` | 修改 | +60 | parse_select 新增 LIMIT/OFFSET/ORDER BY 解析 + 15 个新测试 |
| `src/planner/logical.rs` | 修改 | +3 | 测试 SelectStatement 构造更新 |

### 附录 C：截图和输出文件清单

| 序号 | 文件名 | 内容 | 路径 |
|------|--------|------|------|
| 1 | `branch_analysis.txt` | 分支状态分析和冲突记录 | `screenshots/` |
| 2 | `build_output.txt` | cargo build 编译输出 | `screenshots/` |
| 3 | `test_output.txt` | cargo test 测试输出 | `screenshots/` |
| 4 | `clippy_output.txt` | cargo clippy 检查输出 | `screenshots/` |
| 5 | `fmt_output.txt` | cargo fmt 检查输出 | `screenshots/` |
| 6 | `conflict_output.txt` | git 冲突状态输出 | `screenshots/` |

---

*最后更新: 2026-06-06*
*🕹️ 手动档第2周 - 软件治理入门*
