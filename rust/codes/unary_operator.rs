fn main() {
    let mut a = 5;
    let mut b = 3;
    // rust没有一元加减运算符
    // 这个就等同于 a - (-(-(-(-(b)))))
    print!("{}", a-- - --b); // 2
}
