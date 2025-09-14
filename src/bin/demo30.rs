/// 只可以通过胖指针来操作 Unsize 类型,比如&[T]或&Trait。
/// * 变量、参数和枚举变量不能使用动态大小类型。
/// * 结构体中只有最后一个字段可以使用动态大小类型,其他字段不可以使用。
struct Foo<T: ?Sized> {
    len: usize,
    value: T,  // DST 只能在最后一个字段
}

fn main() {
    
}