## Attributes
attributes是应用于一些module，create或item的元数据，这个元数据可以用于：  
- 代码的条件编译  
- 设置crate的name，version和type(binary or library)  
- 禁用lints(warnings)  
- 启用编译器的特性(宏，全局引入等等)  
- 链接到一个非rust语言的库  
- 标记函数为单元测试  
- 标记函数为某个基准的一部分  

当attributes用于一个完整的crate时，他的语法是`#![crate_attribute]`  
当用于模块或item时，它的语法是`#[item_attribute]`  
attributes可以通过多种方式接受参数：  
- `#[attribute = "value"]`  
- `#[attribute(key = "value")]`
- `#[attribute(value)]`      
- `#[attribute(value1, value2)]`    

#### dead_code  
编译器提供了一个`dead_code`的`link`会警告未使用过的函数，可以使用下面这个attribute来屏蔽  
```rust
#[allow(dead_code)]
fun unsed_function() {}
```

#### crates
- `crate_type`可以告诉编译器crate是binary还是library  
- `crate_name`可以设置create的名字  
cargo的优先级高于上面两个attribute  
```rust
// this is a library
#![crate_type = "lib"]
// this library is named rary
#![crate_name = "rary"]
```
#### cfg
通过两个不同的操作可以实现条件编译  
- `cfg`attribute: `#[cfg(...)]`在属性位置  
- `cfg!`macro: `cfg!(...)`在boolean表达式  
```rust
// this function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_linux() {
}

fn are_you_windows() {
  if cfg!(target_os = "linux") {

  }
}
```
#### custom
像`target_os`这种是rustc默认提供的，如果要自定义的话，需要通过`--cfg`来传给rustc  
```rust
#[cfg(some_condition)]
fn conditional_function() {
  println!("xxx");
}

// $ rustc --cfg some_condition custom.rs && ./custom
```
