fn main() {
    let x = 42;
    let y = Box::new(5);
    println!("{:p}", y);
    let x2 = x;
    println!("{}",x2);
    let y2 = y;
    println!("{:p}",y2);
    println!("{}",x);
}
