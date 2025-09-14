fn main() {
    let x = "hello".to_string();
    match &x[..] {
        "hello" => println!("world"),
        _ => println!("other"),
    }
}