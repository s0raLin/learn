use std::ops::Add;

struct Int(i32);
impl Add<i32> for Int {
    type Output = i32;
    fn add(self, other: i32) -> Self::Output {
        self.0 + other
    }
}

impl Add<i32> for Box<Int> {
    type Output = i32;
    fn add(self, other: i32) -> Self::Output {
        self.0 + other
    }
}

fn main() {
    assert_eq!(Int(3) + 3, 6);
    assert_eq!(Box::new(Int(3)) + 3, 6);
}
