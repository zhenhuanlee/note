- 在`~/.cargo/config`的`[build]`下加`rustflags = ["-Z", "print-link-args"]`可以打印真实的link信息  
- mem::replace  后面的替换前面的，并返回被替换的  
    ```rust
    use std::men;
    
    let mut v: Vec<i32> = vec![1, 2];

    let old_v = mem::replace(&mut v, vec![3, 4, 5]);
    assert_eq!(2, old_v.len());
    assert_eq!(3, v.len());

    // 对于Option，有语法糖take
    let mut x = Some(123);
    x.take()  // mem::replace(x, None)
    ```