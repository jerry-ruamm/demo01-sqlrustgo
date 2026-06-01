# 数据字典

## 学生表 (Student)

| 字段名 | 数据类型 | 说明 | 约束 |
|-------|---------|------|------|
| student_id | VARCHAR(10) | 学号 | PRIMARY KEY |
| name | VARCHAR(50) | 姓名 | NOT NULL |
| gender | ENUM('男', '女') | 性别 | |
| birth_date | DATE | 出生日期 | |
| class | VARCHAR(20) | 班级 | |
| major | VARCHAR(50) | 专业 | |
| phone | VARCHAR(20) | 联系电话 | |

## 课程表 (Course)

| 字段名 | 数据类型 | 说明 | 约束 |
|-------|---------|------|------|
| course_id | VARCHAR(10) | 课程号 | PRIMARY KEY |
| name | VARCHAR(100) | 课程名 | NOT NULL |
| credits | INT | 学分 | DEFAULT 0 |
| semester | VARCHAR(20) | 开课学期 | |

## 成绩表 (Score)

| 字段名 | 数据类型 | 说明 | 约束 |
|-------|---------|------|------|
| id | INT | 自增ID | PRIMARY KEY AUTO_INCREMENT |
| student_id | VARCHAR(10) | 学号 | FOREIGN KEY REFERENCES student(student_id) |
| course_id | VARCHAR(10) | 课程号 | FOREIGN KEY REFERENCES course(course_id) |
| score | DECIMAL(5,2) | 分数 | 0-100 |
| exam_date | DATE | 考试日期 | |