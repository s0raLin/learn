/// 闭包会创建新的作用域，对于环境变量来说有三种捕获方式
/// 1. 对于复制语义类型，以不可变引用&T来捕获
/// 2. 对于移动语义类型，执行移动语义move转移所有权来捕获
/// 3. 对于可变绑定类型，如果在闭包中包含对其修改的操作，则以可变引用&mut来捕获
fn main() {
    let x = 1;
    let closure = || {
        println!("{}", x);
    };
    closure();

    let mut y = 1;
    let mut closure = |i: i32| {
        y += i;
        println!("{}", y)
    };
    closure(32);

    let s = "hello".to_string();
    let join = |i: &str| s + i;
    // println!("{:?}", s);
}
