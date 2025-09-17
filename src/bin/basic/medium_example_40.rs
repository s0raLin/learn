#[derive(Debug)]
struct S(i32);
impl Drop for S {
    fn drop(&mut self) {
        println!("drop for {:?}", self);
    }
}
// 变量遮蔽并不会主动析构原来的变量，它会一直存在，知道函数退出
fn main() {
    let s = S(1);
    println!("create s1 {s:?}");
    let s = S(2);
    println!("create s2 {s:?}")
}
