struct Student {
    name: String,
    age: u32,
    student_id: u32,
}

struct Club {
    name: String,
    members: Vec<Student>,
}

struct Class {
    name: String,
    students: Vec<Student>,
    courses: Vec<Course>,
}

enum Course {
    Math,
    English,
    Science,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();
    let mut courses: Vec<Course> = Vec::new();

    // 创建学生
    let student1 = Student {
        name: String::from("Alice"),
        age: 18,
        student_id: 1,
    };
    students.push(student1);

    let student2 = Student {
        name: String::from("Lily"),
        age: 17,
        student_id: 3,
    };
    students.push(student2);

    // 创建课程
    let course1 = Course::Math;
    let course2 = Course::English;
    courses.push(course1);
    courses.push(course2);

    // 创建班级
    let mut class1 = Class {
        name: String::from("Class A"),
        students,
        courses,
    };

    // 添加新课程到班级
    let course3 = Course::Science;
    class1.courses.push(course3);

    // 读取学生信息
    println!("Class Name: {}", class1.name);
    class1.students.iter().for_each(|student| {
        println!(
            "Name: {}, Age: {}, Student ID: {}",
            student.name, student.age, student.student_id
        )
    });

    // 更新学生信息
    if let Some(student) = class1.students.iter_mut().find(|s| s.student_id == 3) {
        student.age = 19;
    }

    // 读取学生信息
    println!("Class Name: {}", class1.name);
    class1.students.iter().for_each(|student| {
        println!(
            "Name: {}, Age: {}, Student ID: {}",
            student.name, student.age, student.student_id
        )
    });

    // 删除学生信息
    class1.students.retain(|student| student.student_id == 1);

    // 读取学生信息
    println!("Class Name: {}", class1.name);
    class1.students.iter().for_each(|student| {
        println!(
            "Name: {}, Age: {}, Student ID: {}",
            student.name, student.age, student.student_id
        )
    });
    let mut members: Vec<Student> = Vec::new();

    // 创建学生
    let student1 = Student {
        name: String::from("Bob"),
        age: 15,
        student_id: 2,
    };
    members.push(student1);
    // 创建俱乐部
    let club1 = Club {
        name: String::from("Chess Club"),
        members,
    };

    // 打印俱乐部信息
    println!("Club Name: {}", club1.name);
    println!("Members:");
    club1.members.iter().for_each(|member| {
        println!(
            "Name: {}, Age: {}, Student ID: {}",
            member.name, member.age, member.student_id
        )
    });
}
