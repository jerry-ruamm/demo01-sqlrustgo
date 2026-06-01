# 实验报告：结构化方法实践

## 1. 实验概述

本次实验主要完成了学生成绩管理系统的结构化分析与设计，包括：
1. 使用Mermaid语法生成DFD上下文图和Level 1图
2. 使用Mermaid语法生成ER图
3. 设计数据字典
4. 生成SQL建表语句
5. 生成模块结构图
6. 进行Git版本控制

## 2. DFD分析

### 2.1 DFD上下文图

（截图位置：DFD上下文图）

**说明**：上下文图展示了系统与外部实体之间的交互关系，包括学生、教师和教务处三个外部实体，以及系统与数据库之间的数据流动。

### 2.2 DFD Level 1图

（截图位置：DFD Level 1图）

**说明**：Level 1图展示了系统内部的主要功能模块，包括成绩查询模块、成绩录入模块和成绩统计模块，以及它们与数据访问模块和数据库之间的交互。

## 3. ER图设计

（截图位置：ER图）

**说明**：ER图展示了学生、课程和成绩三个实体之间的关系，其中学生和课程之间是多对多关系，通过成绩表作为中间表进行关联。

## 4. 数据字典

### 学生表 (Student)

| 字段名 | 数据类型 | 说明 | 约束 |
|-------|---------|------|------|
| student_id | VARCHAR(10) | 学号 | PRIMARY KEY |
| name | VARCHAR(50) | 姓名 | NOT NULL |
| gender | ENUM('男', '女') | 性别 | |
| birth_date | DATE | 出生日期 | |
| class | VARCHAR(20) | 班级 | |
| major | VARCHAR(50) | 专业 | |
| phone | VARCHAR(20) | 联系电话 | |

### 课程表 (Course)

| 字段名 | 数据类型 | 说明 | 约束 |
|-------|---------|------|------|
| course_id | VARCHAR(10) | 课程号 | PRIMARY KEY |
| name | VARCHAR(100) | 课程名 | NOT NULL |
| credits | INT | 学分 | DEFAULT 0 |
| semester | VARCHAR(20) | 开课学期 | |

### 成绩表 (Score)

| 字段名 | 数据类型 | 说明 | 约束 |
|-------|---------|------|------|
| id | INT | 自增ID | PRIMARY KEY AUTO_INCREMENT |
| student_id | VARCHAR(10) | 学号 | FOREIGN KEY REFERENCES student(student_id) |
| course_id | VARCHAR(10) | 课程号 | FOREIGN KEY REFERENCES course(course_id) |
| score | DECIMAL(5,2) | 分数 | 0-100 |
| exam_date | DATE | 考试日期 | |

## 5. SQL代码

```sql
CREATE TABLE student (
    student_id VARCHAR(10) PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    gender ENUM('男', '女'),
    birth_date DATE,
    class VARCHAR(20),
    major VARCHAR(50),
    phone VARCHAR(20)
);

CREATE TABLE course (
    course_id VARCHAR(10) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    credits INT DEFAULT 0,
    semester VARCHAR(20)
);

CREATE TABLE score (
    id INT AUTO_INCREMENT PRIMARY KEY,
    student_id VARCHAR(10),
    course_id VARCHAR(10),
    score DECIMAL(5,2),
    exam_date DATE,
    FOREIGN KEY (student_id) REFERENCES student(student_id),
    FOREIGN KEY (course_id) REFERENCES course(course_id)
);
```

## 6. 模块结构图

（截图位置：模块结构图）

**说明**：模块结构图展示了系统的分层架构，包括用户界面、业务逻辑层和数据访问层，以及各模块之间的调用关系。

## 7. AI实践心得

使用AI工具生成图表和代码的体验非常高效。通过提供清晰的prompt，AI可以快速生成符合要求的Mermaid图表代码和SQL语句，大大节省了手动编写的时间。同时，AI还可以根据需要调整输出格式和内容，提供多种实现方案。

在使用AI工具时，需要注意以下技巧：
1. 明确指定技术栈和输出格式
2. 提供详细的需求描述
3. 给出具体的示例或参考
4. 对于复杂任务，可以分步骤进行

通过本次实验，我不仅掌握了结构化分析的基本方法，还学会了如何有效地使用AI工具辅助系统设计和开发。