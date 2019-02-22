use std::mem;

fn main() {
    let a;
    let a = a = true;
    // rust没有连等，a=true会返回()，也就是说a = ()，辣眼睛
    println!("{}", mem::size_of_val(&a));  // 0
}
