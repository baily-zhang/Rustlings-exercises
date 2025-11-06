// 深入理解 Place Expression

struct Container {
    data: Vec<u8>,
}

fn main() {
    println!("=== Place Expression 深入解析 ===\n");
    
    // 1. 什么是 Place Expression？
    println!("1. Place Expression 是指向内存位置的表达式");
    let mut container = Container { data: vec![1, 2, 3] };
    
    // container.data 是一个 place expression
    // 它指向内存中 Vec<u8> 的位置
    
    // 2. Place Expression 的上下文相关行为
    println!("\n2. 同一个 place expression 在不同上下文中的行为：");
    
    // a) 在赋值左侧 - 直接操作那个位置
    container.data = vec![4, 5, 6];  // 直接修改那个内存位置
    println!("   a) 赋值左侧: 直接修改内存位置");
    
    // b) 在需要值的上下文 - 尝试移动或拷贝
    let moved = container.data;  // 移动值（因为 Vec<u8> 不是 Copy）
    // println!("{:?}", container.data);  // 错误！已经被移动
    println!("   b) 需要值时: 移动所有权（如果可以）");
    
    // 3. 通过引用访问时的 Place Expression
    println!("\n3. 通过引用访问的 Place Expression：");
    let container2 = Container { data: vec![7, 8, 9] };
    let container_ref = &container2;
    
    // container_ref.data 仍然是 place expression
    // 但因为通过 & 访问，不能移动
    // let cant_move = container_ref.data;  // 错误！不能从引用后面移动
    let can_borrow = &container_ref.data;  // 可以借用
    println!("   通过 &T 访问: 只能借用，不能移动");
    
    // 4. 自动引用和解引用
    println!("\n4. Rust 的自动引用机制：");
    
    // 当 place expression 用在需要引用的地方时
    fn needs_ref(v: &Vec<u8>) {
        println!("   收到引用: {:?}", v);
    }
    
    needs_ref(&container2.data);  // 显式借用
    // 但在方法调用时会自动借用：
    let len = container2.data.len();  // .len() 需要 &self，自动借用
    println!("   方法调用自动借用: len = {}", len);
    
    // 5. Place Expression vs Value Expression
    println!("\n5. Place vs Value Expression 对比：");
    
    let vec = vec![10, 11, 12];
    // vec 是 place expression（指向变量的位置）
    // vec![10, 11, 12] 是 value expression（创建新值）
    
    let x = 5;
    // x 是 place expression
    // 5 是 value expression
    
    println!("   Place: 内存位置（变量、字段、数组元素等）");
    println!("   Value: 实际的值（字面量、函数返回值等）");
    
    // 6. 编译器错误信息的解读
    println!("\n6. 为什么编译器错误显示 'Vec<u8>' 而不是 '&Vec<u8>'：");
    println!("   - Place expression 的类型是它所指向的值的类型");
    println!("   - container_ref.data 指向一个 Vec<u8>");
    println!("   - 所以它的类型是 Vec<u8>（即使你不能移动它）");
    println!("   - 这就是为什么错误信息说 'expected (), found Vec<u8>'");
    
    // 7. 实际应用
    println!("\n7. 在你的代码中：");
    use std::collections::HashMap;
    
    struct Student {
        grades: Vec<u8>,
    }
    
    let mut map = HashMap::new();
    map.insert("Alice", Student { grades: vec![85, 90] });
    
    map.get("Alice").map(|student| {
        // student: &Student
        // student.grades: place expression，类型 Vec<u8>
        // 但只能作为 &Vec<u8> 使用
        
        // 这就是为什么：
        // 1. 类型检查器说它是 Vec<u8>
        // 2. 但你只能借用它
        // 3. .as_slice() 在 &Vec<u8> 上调用（自动借用）
        
        student.grades.as_slice()
    });
    
    println!("   student.grades 是 place expression");
    println!("   类型是 Vec<u8>，但只能作为 &Vec<u8> 使用");
}

// 补充：Place Expression 的完整列表
fn place_expressions_examples() {
    struct S { field: i32 }
    let mut s = S { field: 42 };
    let mut arr = [1, 2, 3];
    let mut vec = vec![1, 2, 3];
    
    // 这些都是 place expressions：
    let _ = s;           // 变量
    let _ = s.field;     // 字段访问
    let _ = arr[0];      // 数组索引
    let _ = vec[0];      // Vec 索引（通过 Deref）
    // let _ = *&s;      // 解引用
    // let _ = (*Box::new(5)); // Box 解引用
}
