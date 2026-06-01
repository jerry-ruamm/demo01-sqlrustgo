实验基本信息
| 项目 | 内容 |
|------|------|
| **实验名称** | Rust 项目初始化与版本控制 |
| **实验周次** | 第 1 周 |
| **实验日期** | 2026 年 3 月 15 日 |
| **学生姓名** | 姚汶辰 |
| **学号** | 202442020122 |
| **班级** | 24软件工程1班 |
| **指导教师** | 李莹 |
---
一、实验目的
（列出本次实验的主要目的，3-5条）
1. 学习 Rust 项目的初始化和基本配置
2. 掌握 Git 版本控制的基本操作，包括分支创建、提交和推送
3. 了解 Rust 项目的构建和测试流程
4. 学习使用 Clippy 和 fmt 等工具进行代码质量检查和格式化
5. 掌握实验报告的撰写规范和要求
---
二、实验环境
2.1 硬件环境
| 项目 | 配置 |
|------|------|
| 计算机型号 | Lenovo Legion R9000P |
| CPU | AMD Ryzen 9 7940HX 16核32线程 |
| 内存 | 16GB DDR5 |
| 硬盘 | WD PC SN560 1TB NVMe SSD |
| GPU | NVIDIA GeForce RTX 4070 Laptop GPU |
2.2 软件环境
| 软件 | 版本 |
|------|------|
| 操作系统 | Microsoft Windows 11 专业版 10.0.26200 |
| Rust | 1.93.1 |
| Git | 2.52.0.windows.1 |
| IDE | Trae IDE |
---
三、实验内容
3.1 任务描述
本次实验主要完成 Rust 项目的初始化、构建、测试以及版本控制管理，包括创建实验分支、提交报告目录等操作。
3.2 实验步骤
#### 步骤1：检查 Rust 安装
**操作命令/代码**：
```bash
rustc --version
cargo --version
```
**执行结果**：
```
rustc 1.93.1 (01f6ddf75 2026-02-11)
cargo 1.93.1 (083ac5135 2025-12-15)
```
**结果分析**：Rust 工具链已成功安装，版本为 1.93.1。
---
#### 步骤2：初始化 Rust 项目
**操作命令/代码**：
```bash
cargo init
```
**执行结果**：
```
Creating binary (application) package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```
**结果分析**：Rust 项目初始化成功，创建了基本的项目结构。
---
#### 步骤3：构建和测试项目
**操作命令/代码**：
```bash
cargo build
cargo test
```
**执行结果**：
```
Compiling sqlrustgo-1 v0.1.0 (D:\workcourse\workware\demo01\sqlrustgo-1)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.88s

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
**结果分析**：项目构建成功，测试通过。
---
#### 步骤4：代码质量检查和格式化
**操作命令/代码**：
```bash
cargo clippy
cargo fmt
```
**执行结果**：
```
Checking sqlrustgo-1 v0.1.0 (D:\workcourse\workware\demo01\sqlrustgo-1)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
```
**结果分析**：代码质量检查通过，代码已成功格式化。
---
#### 步骤5：版本控制操作
**操作命令/代码**：
```bash
git add .
git commit -m "Initial commit"
git checkout -b experiment/week-01-202442020122
mkdir -p reports/week-01
echo "" > reports/week-01/.gitkeep
git add reports/
git commit -m "experiment: submit week-01 report"
git push origin experiment/week-01-202442020122
```
**执行结果**：
```
[master (root-commit) d4bfd7d] Initial commit
 4 files changed, 17 insertions(+)
 create mode 100644 .gitignore
 create mode 100644 Cargo.lock
 create mode 100644 Cargo.toml
 create mode 100644 src/main.rs
Switched to a new branch 'experiment/week-01-202442020122'
[experiment/week-01-202442020122 3a18411] experiment: submit week-01 report
 1 file changed, 0 insertions(+), 0 deletions(-)
 create mode 100644 reports/week-01/.gitkeep
Enumerating objects: 12, done.
Counting objects: 100% (12/12), done.
Delta compression using up to 32 threads
Compressing objects: 100% (6/6), done.
Writing objects: 100% (12/12), 971 bytes | 971.00 KiB/s, done.
Total 12 (delta 1), reused 0 (delta 0), pack-reused 0 (from 0)
remote: Powered by GITEE.COM [1.1.23]
remote: Set trace flag 2849928c
To https://gitee.com/yao-wenchen/sqlrustgo-1.git
 * [new branch]      experiment/week-01-202442020122 -> experiment/week-01-202442020122
```
**结果分析**：Git 操作成功完成，包括创建分支、提交更改和推送至远程仓库。
---
四、实验结果
4.1 完成情况
| 任务 | 完成情况 | 说明 |
|------|----------|------|
| 任务1 | ■ 完成 □ 未完成 | 检查 Rust 安装并初始化项目 |
| 任务2 | ■ 完成 □ 未完成 | 构建和测试项目 |
| 任务3 | ■ 完成 □ 未完成 | 代码质量检查和格式化 |
| 任务4 | ■ 完成 □ 未完成 | 版本控制操作和分支管理 |
| 任务5 | ■ 完成 □ 未完成 | 提交实验报告目录 |
4.2 关键成果
（列出本次实验产出的关键成果）
1. 成功初始化了 Rust 项目，创建了基本的项目结构
2. 完成了项目的构建和测试，确保代码能够正常运行
3. 使用 Clippy 和 fmt 工具进行了代码质量检查和格式化
4. 成功创建了实验分支并推送至远程仓库
5. 建立了实验报告目录结构，为后续实验报告撰写做好准备
4.3 代码提交
| 项目 | 内容 |
|------|------|
| 分支名称 | experiment/week-01-202442020122 |
| 提交哈希 | 3a18411 |
| PR链接 | （待创建） |
---
五、遇到的问题与解决
5.1 问题记录
| 序号 | 问题描述 | 解决方法 | 参考资料 |
|------|----------|----------|----------|
| 1 | 构建项目时遇到链接器错误 | 切换到 GNU 工具链 | Rust 官方文档 |
| 2 | 无法安装 Clippy 工具 | 使用 rustup component add clippy 命令 | Rust 官方文档 |
| 3 | Git 无法跟踪空目录 | 创建 .gitkeep 文件 | Git 官方文档 |
5.2 问题分析
（详细描述遇到的主要问题及解决过程）
1. **链接器错误问题**：在使用 MSVC 工具链构建项目时，遇到了 link.exe 错误，提示缺少 C++ 构建工具。解决方法是切换到 GNU 工具链，使用 `rustup default stable-x86_64-pc-windows-gnu` 命令，成功解决了构建问题。
2. **Clippy 安装问题**：尝试使用 `cargo install clippy` 命令安装 Clippy 时失败，提示 Clippy 不再通过 crates.io 提供。解决方法是使用 `rustup component add clippy` 命令从 Rust 工具链组件中安装。
3. **Git 空目录跟踪问题**：创建的 reports/week-01 目录为空，Git 默认不跟踪空目录。解决方法是在目录中创建一个 .gitkeep 文件，这样 Git 就会跟踪这个目录。
---
六、实验总结
6.1 知识收获
（总结本次实验学到的知识点）
1. 了解了 Rust 项目的基本结构和初始化方法
2. 掌握了 Cargo 工具的基本使用，包括构建、测试和代码质量检查
3. 学习了 Git 版本控制的基本操作，包括分支创建、提交和推送
4. 了解了不同 Rust 工具链（MSVC 和 GNU）的区别和切换方法
5. 掌握了实验报告的撰写规范和要求
6.2 技能提升
（总结本次实验提升的技能）
1. 提升了 Rust 项目的初始化和配置能力
2. 增强了使用 Cargo 工具进行项目管理的技能
3. 提高了 Git 版本控制的操作能力
4. 培养了问题分析和解决的能力
5. 提升了实验报告的撰写能力
6.3 心得体会
（分享实验过程中的心得体会）
1. Rust 是一种安全、高效的编程语言，其工具链和生态系统非常完善
2. 版本控制对于项目管理非常重要，合理使用分支可以有效管理不同阶段的代码
3. 在实验过程中遇到问题时，通过查阅官方文档和搜索引擎可以快速解决问题
4. 实验报告的撰写是对实验过程的总结和反思，有助于加深对知识点的理解
5. 团队协作时，规范的代码风格和版本控制习惯非常重要
6.4 改进建议
（对实验内容或方法的改进建议）
1. 建议在实验前提供更详细的环境配置指南，帮助学生快速搭建开发环境
2. 可以增加一些 Rust 语言的基础语法练习，让学生更好地掌握 Rust 的特点
3. 建议在实验中加入一些实际的项目开发任务，提高学生的实践能力
4. 可以提供更多关于 Git 高级操作的指导，如分支合并、冲突解决等
5. 建议增加代码审查环节，培养学生的代码质量意识
---
七、AI工具使用记录
7.1 AI工具使用情况
| AI工具 | 使用场景 | 效果评价 |
|--------|----------|----------|
| Trae AI | 实验指导和代码辅助 | 优秀 |
| | | |
7.2 AI辅助示例
**输入提示词**：
```
检查rust安装
```
**AI输出结果**：
```
我将检查当前环境中的 Rust 安装情况。

执行 rustc --version 命令，输出：
rustc 1.93.1 (01f6ddf75 2026-02-11)

执行 cargo --version 命令，输出：
cargo 1.93.1 (083ac5135 2025-12-15)

Rust 工具链已经成功安装在您的系统上，并且版本是 1.93.1，这是一个相当新的版本。您现在可以使用 Rust 进行开发了。
```
**使用效果**：
AI 工具能够快速准确地检查 Rust 安装情况，并提供清晰的输出结果。在实验过程中，AI 工具还提供了遇到问题时的解决方案，如切换工具链、安装 Clippy 工具等，大大提高了实验效率。
---
八、参考资料
1. Rust 官方文档：https://doc.rust-lang.org/
2. Git 官方文档：https://git-scm.com/doc
3. Cargo 官方文档：https://doc.rust-lang.org/cargo/
4. Rust 工具链管理：https://rustup.rs/
5. Clippy 官方文档：https://doc.rust-lang.org/clippy/
---
九、教师评语
（教师填写）
| 评价项目 | 得分 |
|----------|------|
| 实验完成度 | /40 |
| 报告规范性 | /20 |
| 问题解决能力 | /20 |
| 创新性 | /10 |
| 总结深度 | /10 |
| **总分** | **/100** |
**教师签名**：________________    **日期**：________________
---
附录
附录A：完整代码
```rust
fn main() {
    println!("Hello, world!");
}
```
附录B：运行日志
```
# 检查 Rust 安装
rustc 1.93.1 (01f6ddf75 2026-02-11)
cargo 1.93.1 (083ac5135 2025-12-15)

# 初始化 Rust 项目
Creating binary (application) package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

# 构建和测试项目
Compiling sqlrustgo-1 v0.1.0 (D:\workcourse\workware\demo01\sqlrustgo-1)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.88s

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

# 代码质量检查和格式化
Checking sqlrustgo-1 v0.1.0 (D:\workcourse\workware\demo01\sqlrustgo-1)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s

# 版本控制操作
[master (root-commit) d4bfd7d] Initial commit
 4 files changed, 17 insertions(+)
 create mode 100644 .gitignore
 create mode 100644 Cargo.lock
 create mode 100644 Cargo.toml
 create mode 100644 src/main.rs
Switched to a new branch 'experiment/week-01-202442020122'
[experiment/week-01-202442020122 3a18411] experiment: submit week-01 report
 1 file changed, 0 insertions(+), 0 deletions(-)
 create mode 100644 reports/week-01/.gitkeep
Enumerating objects: 12, done.
Counting objects: 100% (12/12), done.
Delta compression using up to 32 threads
Compressing objects: 100% (6/6), done.
Writing objects: 100% (12/12), 971 bytes | 971.00 KiB/s, done.
Total 12 (delta 1), reused 0 (delta 0), pack-reused 0 (from 0)
remote: Powered by GITEE.COM [1.1.23]
remote: Set trace flag 2849928c
To https://gitee.com/yao-wenchen/sqlrustgo-1.git
 * [new branch]      experiment/week-01-202442020122 -> experiment/week-01-202442020122
```
附录C：相关截图
（在此粘贴实验过程中的关键截图，例如：
1. Rust 安装检查截图
2. 项目构建和测试结果截图
3. 代码质量检查结果截图
4. Git 操作结果截图
5. 分支推送成功截图）
---
**报告提交日期**：2026 年 3 月 15 日
**学生签名**：姚汶辰