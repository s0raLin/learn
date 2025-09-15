/// ## 静态分发(static_dispatch(&foo))
/// * 在编译时确定调用哪个baz方法
/// * 通过泛型T和where T: Bar约束实现，
/// 编译器知道t的具体类型是Foo,
/// 因此可以直接调用Foo的baz方法
/// * 在main中调用static_dispatch(&foo)时,
/// 编译器会生成直接调用Foo::baz的代码
/// ## 动态分发(dynamic_dispatch)
/// * 在运行时确定调用那个baz方法
/// * 通过trait object & Bar的实现，
/// 编译器不知道t的具体类型，只知道它实现了`Bar trait`
/// * 在运行时，通过查找虚函数表来确定要调用的baz方法
/// * 在main函数中调用dynamic_dispatch(&foo)时，
/// 会将&foo转换为&Bar类型，并在运行时查找正确的baz方法
#[derive(Debug)]
struct Foo;
trait Bar {
    fn baz(&self);
}
impl Bar for Foo {
    fn baz(&self) {
        println!("{:?}", self);
    }
}

fn static_dispatch<T>(t: &T)
where
    T: Bar,
{
    t.baz();
}

//dyn Bar 不是一个具体类型，而是“某个实现了 Bar 的类型”的引用。
fn dynamic_dispatch(t: &dyn Bar) {
    t.baz();
}
fn main() {
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);
}
