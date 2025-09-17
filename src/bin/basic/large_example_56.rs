struct Counter {
    count: usize,
}
impl Iterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 10 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {}

// 类似于namespace，可以使用use将其他模块中的内容引入到当前mod来
mod iterator_demo {
    use super::*;

    #[test]
    fn iter01() {
        let mut counter = Counter { count: 0 };
        for i in &mut counter {
            println!("{}", i);
        }

        //剩余长度的边界信息， 第一个元素表示下限，第二个元素表示上限
        let hint = counter.size_hint();

        println!("{:?}", hint); // (0, None))
    }
    #[test]
    fn iter02() {
        let a = [1, 2, 3];
        let mut iter = a.iter(); // 将数组转换为迭代器
        assert_eq!((3, Some(3)), iter.size_hint());
        iter.next();
        assert_eq!((2, Some(2)), iter.size_hint());
    }

    #[test]
    fn iter03() {
        let mut message = String::from("Hello ");
        // 1.获取迭代器，
        // 2.通过迭代器的size_hint方法获取其长度，
        message.extend(&[' ', 'R', 'u', 's', 't']);
        assert_eq!("hello Rust", &message);
    }
}
