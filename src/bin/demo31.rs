fn test_copy(i: impl Copy) {
    println!("hhh");
}
fn main() {
    let s = "hello".to_string();
    test_copy(s);
}
