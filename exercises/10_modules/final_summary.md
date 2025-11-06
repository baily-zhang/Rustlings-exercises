# Place Expression 终极理解

## 简单来说

当你写 `student.grades` 时：
- **编译器看到的**：这是一个指向 `Vec<u8>` 的位置
- **类型系统标记为**：`Vec<u8>`（存储在那个位置的类型）
- **但实际能做的**：取决于你如何访问这个位置

## 三种情况对比

| 访问方式 | student.grades 能做什么 | 原因 |
|---------|------------------------|------|
| 拥有 `Student` | ✅ 可以移动<br>✅ 可以借用<br>✅ 可以可变借用 | 你拥有所有权 |
| 通过 `&Student` | ❌ 不能移动<br>✅ 只能借用为 `&Vec<u8>`<br>❌ 不能可变借用 | 只有不可变访问权 |
| 通过 `&mut Student` | ❌ 不能移动<br>✅ 可以借用<br>✅ 可以可变借用 | 有可变访问权但不拥有 |

## 为什么会混淆？

```rust
// 通过 &Student 访问时
let _: () = student.grades;  
// 错误: expected (), found Vec<u8>
//      ↑ 编译器说类型是 Vec<u8>

let moved: Vec<u8> = student.grades;
// 错误: cannot move out of student.grades
//      ↑ 但不能当作 Vec<u8> 使用！
```

**原因**：
1. `student.grades` 是 place expression（内存位置）
2. 这个位置存储的是 `Vec<u8>` 类型的值
3. 所以编译器报告类型为 `Vec<u8>`
4. 但因为通过 `&Student` 访问，你没有权限移动它
5. 只能借用为 `&Vec<u8>`

## 实际编程中

```rust
// 在你的代码中
self.students
    .get(name)                           // Option<&Student>
    .map(|student| {                     // student: &Student
        // student.grades 类型显示为 Vec<u8>
        // 但实际上只能作为 &Vec<u8> 使用
        student.grades.as_slice()        // 自动借用为 &Vec<u8>，然后调用 as_slice()
    })
    .unwrap_or(&[])
```

## 一句话总结

**`student.grades` 的类型是 `Vec<u8>`（这是位置中存储的类型），但当通过 `&Student` 访问时，你只能将它作为 `&Vec<u8>` 使用（因为你没有移动的权限）。**

这就是为什么：
- IDE 显示类型是 `Vec<u8>` ✓
- 编译器错误说是 `Vec<u8>` ✓  
- 但你实际只能用作 `&Vec<u8>` ✓

全部都是正确的，只是描述的角度不同！
