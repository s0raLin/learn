use std::process::Output;

#[derive(Debug)]
struct A {
    name: String,
}

impl From<String> for A {
    fn from(str: String) -> Self {
        A { name: str }
    }
}

fn main() {
    let str = "张三".to_string();
    let a: A = A::from("张三");
    println!("{a:?}")
}
