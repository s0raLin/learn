use std::fmt::Debug;

fn match_option<T: Debug>(o :Option<T>) {
    match o {
        Some(i) => println!("{:?}", i),
        None => println!("notinrg"),
    }
}

fn main() {
    let a: Option<i32> = Some(1);
    let b: Option<&str> = Some("hello");
    let c: Option<char> = Some('A');
    let d: Option<u32> = None;
    match_option(a); // 3
    match_option(b); // "hello"
    match_option(c); //'A'
    match_option(d); // nothing




}