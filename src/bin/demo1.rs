fn main() {
    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(ch) = name.pop() {
        println!("{}", ch);
    }

    println!("name: {}", name);
    loop {
        match name.pop() {
            Some(ch) => println!("{}", ch),
            None => break,
        }
    }
}
