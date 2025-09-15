// 变量option为Option<i32>类型，
// 因为i32是复制语义，所以Option<i32>也是复制语义
// 因此在while let匹配后，i的所有权并未转移

fn main() {
    let mut option = Some(0);

    while let Some(i) = option {
        if i > 9 {
            println!("9");
            option = None;
        } else {
            println!("i is {:?}", i);
            option = Some(i + 1);
        }
    }
}
