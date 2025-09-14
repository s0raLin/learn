/// # 所有权借用
fn foo(x: & i32) {

}
fn main() {
    let mut x = 1;
    foo(&x);
    let y = &x;

}