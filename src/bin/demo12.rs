fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}




// struct NotClonable;
#[derive(Debug)]
struct NotClonable;
impl Clone for NotClonable {
    fn clone(&self) -> Self {
        NotClonable
    }
}

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");

    let ret = duplicate(NotClonable);
    println!("{ret:?}",)
}