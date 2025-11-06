use std::collections::HashMap;

pub struct Student {
    pub grades: Vec<u8>,
}

fn main() {
    let mut map = HashMap::new();
    map.insert("Alice", Student { grades: vec![85, 90] });
    
    if let Some(student) = map.get("Alice") {
        // student 是 &Student
        
        // 尝试移动 - 这会失败！
        let moved: Vec<u8> = student.grades;
        println!("This won't compile!");
    }
}
