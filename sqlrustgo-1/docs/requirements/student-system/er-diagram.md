# ER 图

```mermaid
erDiagram
    STUDENT ||--o{ SCORE : "选修"
    COURSE ||--o{ SCORE : "有"
    STUDENT {
        string student_id PK "学号"
        string name "姓名"
        string gender "性别"
        date birth_date "出生日期"
        string class "班级"
        string major "专业"
        string phone "联系电话"
    }
    COURSE {
        string course_id PK "课程号"
        string name "课程名"
        int credits "学分"
        string semester "开课学期"
    }
    SCORE {
        int id PK "自增ID"
        string student_id FK "学号"
        string course_id FK "课程号"
        decimal score "分数"
        date exam_date "考试日期"
    }
```

## 说明

ER图展示了学生成绩管理系统中的三个核心实体及其之间的关系：

1. **实体**：
   - 学生(STUDENT)：存储学生的基本信息，包括学号、姓名、性别、出生日期、班级、专业、联系电话
   - 课程(COURSE)：存储课程的基本信息，包括课程号、课程名、学分、开课学期
   - 成绩(SCORE)：存储学生选课的成绩信息，包括自增ID、学号、课程号、分数、考试日期

2. **关系**：
   - 学生与成绩：一个学生可以选修多门课程（有多个成绩），一个成绩属于一个学生
   - 课程与成绩：一门课程可以被多个学生选修（有多个成绩），一个成绩属于一门课程
