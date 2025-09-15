#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}
impl Person {
    fn new<T: Into<String>>(name: T, age: i32) -> Person {
        Person {
            name: name.into(),
            age,
        }
    }
}

fn main() {
    let p1 = Person::new("Alice", 18);
    println!("{:?}", p1);
}
