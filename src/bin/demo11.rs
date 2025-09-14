#[derive(Debug)]
struct Foo(String);

impl From<u32> for Foo {
    fn from(x: u32) -> Foo {
        Foo(format!("{}转换为字符串", x))
    }
}
impl From<bool> for Foo {
    fn from(b: bool) -> Foo {
        Foo(format!("{}转换为字符串", b))
    }
}

fn main() {
    let from_int = Foo::from(123);
    let from_bool = Foo::from(true);
    println!("{from_int:?}, {from_bool:?}");
}