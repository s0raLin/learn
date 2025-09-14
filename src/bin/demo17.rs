fn main() {
    let a = [1,2,3,4,5];
    let b = &a;
    println!("{:p}", b);

    let mut c = vec![1,2,3];
    let d = &mut c;
    d[0] = 3;
    println!("{:p}", d);
    println!("{:p}", &c);
    println!("{:p}", &c);
}