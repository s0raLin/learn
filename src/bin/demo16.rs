fn main() {
    let a = "hello";
    let b = "hello".to_string();

    let ohter = a;
    println!("{:?}", ohter);
    let ohter = b;
    println!("{:?}", ohter);
    //这时在使用b就不行了
    println!("{}", a);
    // println!("{}",b);
}
