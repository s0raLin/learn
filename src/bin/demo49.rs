/// # 函数
/// 函数本身是独立的词法作用域。
/// * 当复制语义类型作为函数参数时，会按位复制
/// * 当移动语义作为函数参数时，会转移所有权
fn foo(s: String) -> String {
    let w = "world".to_string();
    s + &w
}

fn main() {
    let s = "hello".to_string();
    let ss = foo(s);
    println!("{}", ss);
    // println!("{}", s);
}
