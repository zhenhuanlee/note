trait Trait {
    fn f(&self);
    fn g(&self);
}

struct S;

impl S {
    fn f(&self) {
        print!("1");
    }

    fn g(& mut self) {
        print!("1");
    }
}

impl Trait for S {
    fn f(&self) {
        print!("2");
    }

    fn g(&self) {
        print!("2");
    }
}

fn main() {
    // 总是优先调用内部方法
    S.f();
    // 参数的优先级貌似是 self, &self, &mut self
    // mut self好像和self没啥区别
    S.g();
}
