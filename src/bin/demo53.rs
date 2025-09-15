/// 'b:'a的意思是泛型生命周期参数‘b的存活时间长于’a
/// 函数间传入和返回的借用必须相关联，
/// 并且返回借用生命周期必须比出借放的生命周期长，
/// 所以'b:'a中的'a表示返回引用的生命周期，必须不长于'b(出借方)的生命周期
fn the_longest<'a, 'b: 'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn main() {
    let s1 = String::from("Rust");
    let s1_r = &s1;
    {
        let s2 = String::from("C");
        let res = the_longest(s1_r, &s2);
        println!("{} is the longest", res);
    }
}
