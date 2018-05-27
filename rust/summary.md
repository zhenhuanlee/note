## 变量声明
```rust
let y = 6;  // 普通声明，不可变
let mut x = 5;  // 可变声明
 
x = "123"; // 会报错
let y = "asdf";  // 正确  叫：shadowing

const MAX_POINTS: u32 = 100_00; // 常量声明，感觉像全局变量
```

## 数据类型
主要分单一和复合类型  
#### 单一
- Integer(i8, u8...)  
- Float  
```rust
let x = 2.0;      // f64
let y: f32 = 3.0; // f32
```

#### 运算
```rust
fn average(values: &[f64]) -> f64 {
    let sum: f64 = sum(values);
    let size: f64 = len(values) as f64;
    sum / size
    // 有点像parse()
}
```




