use std::vec;

fn main() {
    let x = 5;
    let y = x;
    assert_eq!(x,5);
    assert_eq!(y,5);

    let v1 = vec![1,2,3];
    let v2 = v1;
    assert_eq!(v2,vec![1,2,3]);
}