use std::rc::Rc;

fn main() {
    let r1 = Rc::new(1);
    println!("{:#?}",Rc::strong_count(&r1));

    let r2 = Rc::clone(&r1);
    println!("{:#?}",Rc::strong_count(&r2));
}
// Rc 是


mod test {
    
}