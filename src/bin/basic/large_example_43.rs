// 显然引用语义类型不能实现Copy,
// 但可以实现Clone的clone方法，
// 以实现深复制，在需要的时候可以显式调用
#[derive(Clone)]
struct A {
    a: i32,
    b: Box<i32>,
}

#[derive(Copy, Clone)]
struct B {
    i: i32,
    j: i32,
}
fn main() {
    let a = A {
        a: 1,
        b: Box::new(2),
    };
    // Rust 并不会在赋值时自动调用 clone。
    // 只有满足 Copy trait 的类型才可以在赋值时按值复制
    //，而 Box<i32> 并不是 Copy 类型。
    let b = a.clone();
    println!("{}", a.a);

    let b: B = B { i: 1, j: 2 };
    let c = b;
    assert_eq!(b.i, 1);
    assert_eq!(b.j, 2);
    assert_eq!(c.i, 1);
    assert_eq!(c.j, 2);
}
