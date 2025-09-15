fn duplicate<T, U>(a: T, b: U) -> (T, U)
where
    T: Clone,
    U: Clone,
{
    (a.clone(), b.clone())
}

fn main() {
    let a = String::from("hello");
    let b = String::from("world");
    let ret = duplicate::<String, String>(a, b);
    println!("{} {}", ret.0, ret.1);
}
