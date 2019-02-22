#[derive(Debug)]
struct Guard {
    name: &'static str,
}

impl Drop for Guard {
    fn drop(&mut self) {
        print!("{:?}", self.name);
    }
}

fn main() {
    let _guard = Guard{name: "a"};
    print!("3");
    let _ = Guard{name: "b"};
    print!("2");
}

// 3b2a
