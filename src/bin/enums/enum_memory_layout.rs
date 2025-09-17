#[derive(Debug)]
enum MyEnum {
    A(i32),
    B(f64),
    C(bool),
}

fn main() {
    // 创建一个枚举变量
    let mut e = MyEnum::A(42);

    // 栈上存储：tag + payload
    println!("Initial: {:?}", e);

    // 改变为 B 变体，payload 内存被覆盖
    e = MyEnum::B(3.14);
    println!("After change: {:?}", e);

    // 改变为 C 变体，payload 内存再次被覆盖
    e = MyEnum::C(true);
    println!("After change again: {:?}", e);

    // 模拟底层访问
    use std::mem;
    println!("Size of MyEnum: {}", mem::size_of::<MyEnum>()); // 包含 tag + payload
}
