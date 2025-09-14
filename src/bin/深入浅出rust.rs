fn main() {}

// Format格式说明
mod format格式 {
    use super::*;

    #[test]
    fn test_format() {
        //println!是一个宏，来自标准库std::fmt模块
        //类似于format! 返回String
        println!("{}", 1); // 默认用法，打印Display
        println!("{:o}", 9); // 八进制
        println!("{:x}", 255); //十六进制(小写)
        println!("{:X}", 255); //十六进制(大写)
        println!("{:p}", &0); //指针
        println!("{:b}", 15); //二进制
        println!("{:e}", 10000_f32); //科学计数(小写)
        println!("{:E}", 10000_f32); //科学计数(大写)
        println!("{:?}", "test"); //打印Debug
        println!("{:#?}", ("test1", "test2")); //带换行和缩进的Debug打印
        println!("{a} {b} {b}", a = "x", b = "y"); //命名参数
    }
}


// 变量
mod val {
    use super::*;

    #[test]
    fn test_val() {
        let mut v = Vec::new(); // v必须是mut修饰，因为需要对它写入数据
        v.push(1);
        v.push(2);
        v.push(3);

        let v = v; // 从这里v变成了只读变量，可读写变量v已经被遮蔽，无法被访问
        for i in &v {
            println!("{}", i);
        }
    }


}

// 标准库中常见的trait
mod std_trait {

    use std::fmt::{Display, Error, Formatter};
    use super::*;

    // 1. std::fmt::Display
    // 用在类似println!()这样的地方
    // 只有实现了Display trait的类型，才能用{}格式打印出来
    #[derive(Debug)]
    struct T {
        field1: i32,
        field2: i32,
    }

    impl Display for T {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            write!(f, "{{ field1:{}, field2:{} }}", self.field1, self.field2)
        }
    }


    #[test]
    fn main() {
        let var = T {field1: 1, field2: 2};
        println!("{}", var);
        println!("{:?}", var);
        println!("{:#?}", var);
    }
    // 只有实现了Display trait的类型，才能用{} 格式打印出来,
    // 只有实现了Debug trait的类型，才能用{:?} 和{:#?}格式控制打印出来
    // * Display 假定这个类型可以用utf-8格式的字符串表示，是给用户看的
    // * 标准库还有一个常用的trait叫std::string::ToString,
    // 对于所有实现了Display trait的类型，都自动实现了这个ToString trait，
    // 包含一个to_string(&self)->String方法,可以用它格式化出一个字符串
    // * Debug主要用于调试，建议将所有作为API的公开类型都实现这个trait,以方便调试
}



// PartialOrd/Ord/PartialEq/Eq
// PartialEq  ->  Eq
// PartialOrd ->  Ord
mod total_order {
    use super::*;

    // 由于Nan的存在，浮点数并不具备全序关系
    // * 对于a<b一定有!(a>b); 反之，若a>b,则一定有!(a<b)，则称为反对称性
    // * 对于a<b且b<c 则a<c; 称为传递性
    // * 对于X中所有元素，都存在a<b或a>b或a==b; 三者必居其一,则称为完全性
    // 如果集合X中所有元素都满足前两条特征，则称X为"偏序",同时具备所有特征，则称X为"全序"
    #[test]
    fn test() {
        let nan = f64::NAN;
        let x = 1.0;
        println!("{}", nan<x); // false
        println!("{}", nan>x); // false
        println!("{}", nan==x);// false

        // 因此，Rust设计了两个trait来描述这样的状态:
        // * 表示偏序 std::cmp::PartialOrd,
        // * 表示全序 std::cmp::Ord
        // partial_cmp() 返回值类型是Option<Ordering>。
        // 只有Ord trait里的cmp() 才会返回一个确定的Ordering.
        // f32和f64都只实现了PartialOrd,而没有实现Ord。

        let int_vec = [1, 2, 3];
        let biggest_int = int_vec.iter().max();

        let float_vec = [1.0, 2.0, 3.0];
        // let biggest_float = float_vec.iter().max();

        // PartialEq 和 Eq的作用是比较相等关系
    }

}


//Sized
mod sized {
    use super::*;
    // 用户不能针对自己的类型impl这个trait,
    // 一个类型是否满足Sized约束完全是由编译器推导的
    // 在rust中，编译阶段能确定大小的类型，都满足Sized约束
    // 不定长数组的长度在编译阶段是未知的，不满足Sized约束
}


mod default {
    use super::*;

    //Default trait
    //它只包含一个静态函数default()返回Self类型。
    //标准库中很多类型都实现了这个trait,相当于提供了一个类型的默认值
}



// Range
mod range {
    use std::ops::{Range, RangeTo};
    use super::*;


    //Range代表一个区间[begin..end)
    //这个语法实际上生成的是std::ops::Range<_>类型的变量
    // 1..10 代表[1,10)这个区间
    //Rust中有几种Range
    // * std::ops::RangeFrom -> 语法start.. 代表没有结束范围
    // * std::ops::RangeTo -> 语法..end 代表没有起始范围
    // * std::ops::RangeFull -> 语法.. 代表没有上下限制范围 对于无符号数[0,+无穷)

    fn print_slice(arr: &[i32]) {
        println!("length:{:?}", arr.len());
        for item in arr {
            print!("{}\t", item);
        }
        println!("");
    }
    #[test]
    fn main() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        print_slice(&arr[..]); // full range

        let slice = &arr[2..]; // RangeFrom
        print_slice(slice);

        let slice2 = &arr[..2]; //RangeTo
        print_slice(slice2);
    }
}
