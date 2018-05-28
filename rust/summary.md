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

#### 复合类型
##### tuple
```rust
    let tup1: (i32, f64, u8) = (12, 2.2, 9);
    println!("{:?}", tup1);

    let x = tup1.0;
    let y = tup1.1;

    println!("x: {}, y: {}", x, y);
```

##### array
- 大小不可改（vector可以）  
```rust
let a = [1,2,3,4,5];

println!("array: {:?}, first: {}, second: {}", a, a[0], a[1]);
```

#### function
- 函数体
```rust
let x = 5;
let y = {
    let x = 3;
    x + 1
};

println!("x: {}, y: {}",x, y);
```

#### control flow
##### 可以有赋值操作
```rust
let condition = true;
let number = if condition {
    5
} else {
    6
};

println!("number: {}", number);
```

