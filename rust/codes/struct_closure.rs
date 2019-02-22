struct S {
    f: fn(),
}

impl S {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    let s = S {
        f: || print!("1"),
    };

    s.f();   // 2
    (s.f)(); // 1
}
