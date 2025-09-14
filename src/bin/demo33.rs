struct S(i32);
trait A {
    fn test(&self, i: i32);
}
trait B {
    fn test(&self, i: i32);
}
impl A for S {
    fn test(&self, i: i32) {
        println!("Form A: {}",i)
    }
}
impl B for S {
    fn test(&self, i: i32) {
        println!("Form B: {}",i)
    }
}
fn main() {
    let s = S(1);
    A::test(&s, 1);
    B::test(&s, 2);
    <S as A>::test(&s, 3);
    <S as B>::test(&s, 4);
}