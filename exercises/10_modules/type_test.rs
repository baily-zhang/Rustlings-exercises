use std::collections::HashMap;

pub struct Student {
    pub name: String,
    pub grades: Vec<u8>,
}

pub struct StudentGrades {
    pub students: HashMap<String, Student>,
}

impl StudentGrades {
    pub fn get_grades(&self, name: &str) -> &[u8] {
        self.students
            .get(name)
            .map(|student| {
                // 测试1: 尝试移动所有权 - 这应该失败
                // let moved: Vec<u8> = student.grades;  // 取消注释会报错
                
                // 测试2: 明确标注类型
                let grades_ref: &Vec<u8> = &student.grades;
                println!("Type test passed: &Vec<u8>");
                
                // 测试3: 使用 std::any 来检查实际类型
                use std::any::type_name;
                println!("Type of student: {}", type_name::<&Student>());
                println!("Type of &student.grades: {}", type_name::<&Vec<u8>>());
                
                // 测试4: 故意写错类型让编译器告诉我们
                // let _: () = student.grades;  // 这行会显示实际类型
                
                grades_ref.as_slice()
            })
            .unwrap_or(&[])
    }
    
    pub fn test_place_expression(&self, name: &str) {
        if let Some(student) = self.students.get(name) {
            // student 是 &Student
            
            // 这是一个 "place expression"
            // student.grades 在不同上下文中表现不同：
            
            // 1. 在需要引用的上下文中，自动变成 &Vec<u8>
            let _ref: &Vec<u8> = &student.grades;
            
            // 2. 但类型推断时显示为 Vec<u8> (place expression)
            // let _: () = student.grades;  // 错误信息会说是 Vec<u8>
            
            // 3. 实际上不能移动所有权
            // let _moved: Vec<u8> = student.grades;  // 这会失败！
            
            println!("Place expression behavior demonstrated");
        }
    }
}

fn main() {
    let mut tracker = StudentGrades {
        students: HashMap::new(),
    };
    
    tracker.students.insert(
        "Alice".to_string(),
        Student {
            name: "Alice".to_string(),
            grades: vec![85, 90],
        }
    );
    
    // 运行测试
    tracker.get_grades("Alice");
    tracker.test_place_expression("Alice");
    
    println!("\n关键理解：");
    println!("1. student.grades 是一个 'place expression'");
    println!("2. 编译器错误显示 Vec<u8> 是因为它显示的是 place 的类型");
    println!("3. 但实际上你不能移动它，只能借用它");
    println!("4. 在需要值的上下文中，它会自动变成 &Vec<u8>");
}
