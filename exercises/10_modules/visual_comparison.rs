// 可视化对比：Place Expression 在不同情况下的表现

use std::collections::HashMap;

struct Student {
    name: String,
    grades: Vec<u8>,
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║         Place Expression 行为对比                         ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
    
    // 场景 1: 拥有所有权时
    println!("📦 场景 1: 拥有所有权的 Student");
    println!("─────────────────────────────────");
    let mut owned_student = Student {
        name: "Alice".to_string(),
        grades: vec![85, 90],
    };
    
    // ✅ 可以移动
    let moved_grades = owned_student.grades;
    println!("✅ 可以移动: let moved = owned_student.grades");
    println!("   moved_grades = {:?}", moved_grades);
    
    // 重新赋值用于后续测试
    owned_student.grades = vec![95, 100];
    
    // ✅ 可以借用
    let borrowed = &owned_student.grades;
    println!("✅ 可以借用: let borrowed = &owned_student.grades");
    println!("   borrowed = {:?}", borrowed);
    
    // ✅ 可以可变借用
    let mutable_borrow = &mut owned_student.grades;
    mutable_borrow.push(88);
    println!("✅ 可以可变借用并修改");
    println!("   修改后 = {:?}\n", owned_student.grades);
    
    // 场景 2: 通过不可变引用访问
    println!("🔒 场景 2: 通过 &Student 访问");
    println!("────────────────────────────────");
    let student_ref = &owned_student;
    
    // ❌ 不能移动
    // let cant_move = student_ref.grades;  // 错误！
    println!("❌ 不能移动: let cant_move = student_ref.grades");
    println!("   编译错误: cannot move out of `student_ref.grades`");
    
    // ✅ 可以借用
    let can_borrow = &student_ref.grades;
    println!("✅ 可以借用: let can_borrow = &student_ref.grades");
    println!("   can_borrow 类型 = &Vec<u8>");
    println!("   值 = {:?}", can_borrow);
    
    // ❌ 不能可变借用
    // let cant_mut = &mut student_ref.grades;  // 错误！
    println!("❌ 不能可变借用: &mut student_ref.grades");
    println!("   编译错误: cannot borrow as mutable\n");
    
    // 场景 3: HashMap 中的典型用法
    println!("🗺️  场景 3: HashMap.get() 返回的 Option<&Student>");
    println!("──────────────────────────────────────────────────");
    let mut map = HashMap::new();
    map.insert("Bob", Student {
        name: "Bob".to_string(),
        grades: vec![75, 80, 85],
    });
    
    if let Some(student) = map.get("Bob") {
        println!("student 类型: &Student");
        println!("student.grades 是 place expression");
        
        // 类型"看起来"是 Vec<u8>
        println!("\n📝 IDE/编译器显示:");
        println!("   student.grades 的类型 = Vec<u8>");
        
        // 但实际只能作为 &Vec<u8> 使用
        println!("\n🔧 实际可用操作:");
        println!("   ✅ &student.grades → &Vec<u8>");
        println!("   ✅ student.grades.len() → 自动借用为 &Vec<u8>");
        println!("   ✅ student.grades.as_slice() → 自动借用");
        println!("   ❌ let v: Vec<u8> = student.grades → 不能移动!");
        
        // 实际使用
        let grades_slice = student.grades.as_slice();
        println!("\n实际值: {:?}", grades_slice);
    }
    
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║                        总结                               ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!("
🎯 关键理解：
1. student.grades 是 place expression，指向内存位置
2. 其类型标注为 Vec<u8>（位置中存储的类型）
3. 但通过 &Student 访问时，只能获得 &Vec<u8>
4. 这就是为什么：
   - 编译器错误说 \"found Vec<u8>\"
   - 但你不能真正获得 Vec<u8> 的所有权
   - 只能借用它作为 &Vec<u8>

💡 记住：Place Expression 的类型 ≠ 你能获得的值的类型
   当通过引用访问时，你只能借用，不能移动！
");
}
