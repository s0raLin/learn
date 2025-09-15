fn counter(i: i32) -> Box<dyn Fn(i32) -> i32> {
    // 放入了Box<T>中，因为闭包的大小在编译期是未知的，
    // Rust2018版本中也可以使用impl Trait语法 impl Fn(i32)->i32
    Box::new(move |n: i32| n + i)
}

fn main() {
    let f = counter(3);
    assert_eq!(4, f(1))
}
