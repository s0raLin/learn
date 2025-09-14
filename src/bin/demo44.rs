// 虽然结构体A的成员都是复制语义类型，
// 但Rust并不会默认为其实现Copy,需有手动实现
#[derive(Debug, Clone, Copy)]
struct A {
    a: i32,
    b: u32,
}
fn main() {
    let a = A { a: 1, b: 2 };
    let b = a;
    println!("{:?}",a);
}