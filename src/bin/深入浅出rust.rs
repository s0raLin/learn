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

    // ref 和 mut
    // 之所以在某些时候使用ref,是因为在模式匹配时有可能发生变量的所有权转移
    // 使用ref就是为了避免出现所有权的转移
    #[test]
    fn demo() {
        let x = 5_i32; // i32
        let x = &5_i32; // &i32
        let ref x = 5_i32; // &i32
        let ref x = &5_i32; // &&i32
    }
    // 上述代码可见，ref是模式的一部分，只能出现在赋值号左边，
    // 而&符号是借用运算符，是表达式的一部分，只能出现在赋值号右边

}

// 标准库中常见的trait
mod std_trait {

    use super::*;
    use std::fmt::{Display, Error, Formatter};

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
        let var = T {
            field1: 1,
            field2: 2,
        };
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
        println!("{}", nan < x); // false
        println!("{}", nan > x); // false
        println!("{}", nan == x); // false

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
    use super::*;
    use std::ops::{Range, RangeTo};

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

mod macro_demo {

    use std::collections::HashMap;

    /// 1. 添加语句块
    /// ```rust
    /// macro_rules! hashmap {
    ///     ($($key:expr => $val: expr),*) => {
    ///         {
    ///             let mut hashmap = HashMap::new();
    ///             hashmap.insert($key, $val);
    ///             hashmap
    ///         }
    ///     }
    /// }
    /// 2. 重复的语法
    ///     +模式代表一个或多个重复
    ///     * 模式代表0个或多个重复
    /// ```
    macro_rules! hashmap {

        ($($key: expr => $val: expr),* $(,)?) =>{
            {
                let mut hashmap = HashMap::new();
                $(hashmap.insert($key, $val);)*
                hashmap
            }
        };
    }

    #[test]
    fn main() {
        let map = hashmap![
            'a' => 1,
            'b' => 2,
            'c' => 3
        ];
        println!("{:#?}", map);
    }
}

// 析构函数
mod drop {
    use super::*;

    struct T(i32);

    impl Drop for T {
        fn drop(&mut self) {
            println!("析构函数被调用");
        }
    }

    #[test]
    fn main() {
        {
            let t = T(100);
        }
        println!("Ok");
    }
}

// 生命周期标记
mod lifetime {
    /// # 函数的生命周期标记
    // 对于一个函数内部的生命周期进行分析，Rust编译器可以很好地解决。
    // 但是当生命周期跨函数时，就需要一种特殊的生命周期标记符号了
    /// 代码示例
    ///```rust
    /// struct T {
    ///     number: i32,
    /// }
    ///
    /// fn test<'a>(arg: &'a T) -> &'a i32 {
    ///     &arg.number
    /// }
    ///
    /// fn main() {
    ///     let t = T { number: 0 };  // t
    ///     let x = test(&t);         // x
    ///     println!("{:?}", x);      //
    /// }                             // x,t
    /// ```
    //
    // 如果生命周期'a比'b更长或相等，则记为'a:'b 意思是'a至少不会比'b短，
    // 对于借用指针类型来说，如果&'a是合法的, 那么'b作为'a的一部分也是合法的
    // 另外, 'static是一个特殊的生命周期，它代表的是这个程序从一开始到结束的整个阶段。
    // 这就意味着任意一个生命周期'a都满足'static: 'a

    // 以上代码中我们把变量t的生命周期标记为't,那么这个生命周期't实际上是变量t从出生到死亡的区间，
    // 1.当函数被调用时，它传入实际的参数是&t(指向t的引用)，那么可以说在调用的时候这个泛型'a被实例化为了't
    // 2.根据函数签名，基于返回类型的生命周期与参数是一致的，可以推理出test函数的返回类型是&'t i32
    // 如果我们把x的生命周期记为'x,那么'x代表的就是变量x从出生到死亡的区间
    // 1.这条语句(let x = text(&t))，实际上把&'t i32类型的变量赋值给&'x i32类型的变量
    // 这段代码是合理的，因为这两个的关系是‘t: 'x，
    // test返回的那个指针't这个生命周期范围内都是合法的，在一个被't包围的更小范围生命周期内也是合法的

    //接下来将上述例子稍作更改，让test有两个生命周期参数，其中一个给函数参数使用，另一个给返回值使用
    /// ```rust
    /// fn test<'a, 'b>(arg: &'a T)-> &'b i32 {
    ///     &arg.number
    /// }
    /// ```
    ///
    // 编译果然出了问题，报了生命周期错误。这是为什么呢？
    // 因为将&'a i32类型赋值给&'b i32类型。
    // ‘a和'b有什么关系？答案是什么关系也没有，所以编译器认为这个赋值是错误的
    // 怎么修复呢?指定'a: 'b 就可以，'a比'b活得长，自然&'a i32 类型赋值给&'b i32类型是没问题的
    /// 验证如下
    /// ```rust
    /// fn test<'a, 'b>(arg: &'a T)-> &'b i32
    /// where 'a: 'b {
    ///     &arg.number
    /// }
    /// ```
    ///
    // 经过这样改写后，我们可以认为，在test函数被调用的时候，生命周期参数'a和'b被分别实例化为了't和'x。
    // 它们刚好满足where条件中的't: 'x约束。
    // 而&arg.number这条表达式的类型是&'t i32, 返回值要求是&'x i32的类型，可见这也是合理的。
    // 所以test函数的生命周期检查可以通过
    // 上述示例是比较难理解的地方，
    /// 以下写法也是可行的。
    /// ```rust
    /// fn test<'a>(arg: &'a T)->&'a i32
    /// fn test<'a, 'b>(arg: &'a T)->&'b i32 where 'a: 'b
    /// ```
    // 这样写的关键是，Rust的引用类型是支持“协变”的。
    // 在编译器眼里，生命周期就是一个区间，生命周期参数就是一个普通的泛型参数。
    // 它可以被特化为某个具体的生命周期。

    /// 再看一个例子, 它有两个引用参数，共享一个生命周期标记。
    fn select<'a>(arg1: &'a i32, arg2: &'a i32) -> &'a i32 {
        if *arg1 > *arg2 { arg1 } else { arg2 }
    }
    #[test]
    fn main() {
        let x = 1;
        let y = 2;
        let selected = select(&x, &y);
        println!("{}", selected);
    }
    // select 这个函数引入了一个生命周期标记，
    // 两个参数以及返回值都是用的这两个生命周期标记，
    // 同时需要注意到，在调用时，传递的实参其实具备不同的生命周期的。
    // x的生命周期明显大于y的生命周期，&x可存活的范围要大于&y可存活的范围.
    // 我们把它们的实际生命周期分别记录为‘x和'y.
    // select函数的形式参数要求的是同样的生命周期，而实际上参数是两个不同的生命周期的引用。
    // 这个类型之所以可以匹配成功，是因为生命周期的协变特性，
    // 编译器可以把&x和&y的生命周期都缩小到某个生命周期’a以内，且满足'x： ‘a, 'y: 'a
    // 返回的selected变量具备'a生命周期，也没有超过'x和'y的范围，所以最终的生命周期检查通过

    /// # 类型的生命周期标记
    //如果自定义类型中有成员包含生命周期参数，那么这个自定义类型必须要有生命周期参数。

    /// 示例如下
    /// ```rust
    /// struct Test<'a> {
    ///     number: &'a str
    /// }
    /// // 在使用impl时，也必须要先声明后使用
    /// impl<'t> Test<'t> {
    ///     fn test<'a>(&self, a: &'a str) {
    ///
    ///     }
    /// }
    /// ```
    ///
    ///
    // impl后的't是用于声明生命周期参数的,Test<'t>中的't是在类型中使用这个参数，
    // 如果有必要的话方法中还能引入新的泛型参数
    // 如果在泛型约束中有where T: ‘a 之类的条件，其意思是：
    // * 类型T的所有生命周期参数必须大于等于'a。 需要特别说明的是：
    // 若是有where T: 'static的约束，则表示类型T里面不包含任何指向短生命周期的借用指针
    // 意思是要么完全不包含任何借用，要么可以指向'static的借用指针。

    /// # 省略生命周期标记
    // 在某些情况下，Rust运行我们在写函数时省略显式生命周期标记。
    // 编译器会通过一定的固定规则为参数和返回值指定合适的生命周期。
    /// 代码示例
    /// ```rust
    /// fn get_str(a: &String) -> &str {
    ///     s.as_ref()
    /// }
    /// //实际上等同于下面的代码
    /// fn get_str<'a>(s: &'a String) -> &'a str {
    ///     a.as_ref()
    /// }
    /// ```
    fn get_str<'a>(s: &'a String) -> &'static str {
        println!("get_str : {}", s);
        "hello world"
    }

    #[test]
    fn test() {
        let c = String::from("hello");
        let str: &'static str = get_str(&c);

        println!("{}", str);
    }
    // error[E0597]: `c` does not live long enough
    // 按道理，变量x应该指向一个'static生命周期的变量，根本不是指向c变量，
    // 它的存活时间足够长，为什么编译器没发现这一点呢？
    // 这是因为编译器对于省略掉的生命周期，不是用的自动推理策略，而是用的几个简单的固定规则
    // 这跟类型推导不一样，当省略变量的类型时，编译器试图通过变量的使用方式推导出变量的类型。
    // 而对于省略掉的生命周期参数，编译器的处理方式就简单粗暴很多，以下就是省略的生命周期被自动补全的规则
    // * 每个带生命周参数的输入参数，每个对应不同的生命周期参数
    // * 如果只有一个输入参数带生命周期参数，那么返回值的生命周被指定为这个参数
    // * 如果有多个输入参数带生命周期参数，但其中有&self、&mut、self,那么返回值的生命周期就会被指定为这个参数
    // * 以上都不满足，就不鞥你自动补全返回值的生命周期参数

    // 尝试修复上述错误
    fn get_str2<'a>(s: &'a String) -> &'static str {
        println!("get_str2 : {}", s);
        "hello world"
    }
    #[test]
    fn demo2() {
        let c = String::from("hello");
        let str2 = get_str2(&c);
        println!("{}", str2);
    }
}

// 借用检查
mod err {
    use super::*;

    // Rust的编译器错误列表中，从E0499到E0509,都在讲同一件事情。
    // 它们主要关心的是共享和可变之间的关系。
    // 共享不可变，可变不共享,是所有这些编译错误共同遵循的法则

    /// # 编译错误示例
    /// ## 示例一
    #[test]
    fn demo01() {
        let i = 0;
        let p1 = &i;
        let p2 = &i;
        println!("i: {}, p1: {}, p2: {}", i, p1, p2);
    }
    // 上面这段代码是编译通过的。
    // 其中变量绑定的i、p1、p2指向同一个变量，我们可以通过不同的Path访问同一块内存(i, *p1, *p2)
    // 它们都只有只读的权限，它们存在共享，不存在可变，因此它一定是内存安全的

    /// ## 示例二
    /// ```rust
    /// #[test]
    /// fn demo02() {
    ///     let mut i = 0;
    ///     let p1 = &i;
    ///    i = 1;
    ///     println!("p1: {}", p1);
    /// }
    /// ```
    /// `error[E0506]`: cannot assign to `i` because it is borrowed
    /// 上面这段代码存在问题:
    /// 在存在只读借用的情况下，变量绑定i和p1已经互为alias(共享) 因此必须避免可变
    /// 这段代码违反了“共享不可变”的原则。

    /// ## 示例三
    #[test]
    fn demo03() {
        let mut i = 0;
        let p1 = &mut i;
        *p1 = 1;
        println!("i: {}", i);
    }
    // 上面这段代码可以编译通过。
    // 不是应该违反了“共享不可变”的原则吗?其实不是。这段代码中不存在共享。
    // 在可变借用存在时，编译器认为原来的变量绑定i已经被冻结。不可通过i读写变量。
    // 此时有且只有p1这个入口可以读写变量(按道理说旧版本是这样的，Rust2018引入了NLL的新借用检查器)：
    #[test]
    fn test03() {
        let mut i = 0;
        let p1 = &mut i;
        *p1 = 1;
        let x = i; //通过i读变量
    }

    /// ## 示例四
    /// 同时创建两个可变借用的情况
    /// ```rust
    /// #[test]
    /// fn demo04() {
    ///     let mut i = 0;
    ///     let p1 = &mut i;
    ///     let p2 = &mut i;
    ///
    ///     *p1 = 2;
    ///
    /// }
    ///```rust
    // error[E0499]: cannot borrow `i` as mutable more than once at a time
    // 因为p1和p2都是可变借用，它们都指向同一个变量，
    // 而且都有修改权限，这是Rust不允许的情况，因此编译无法通过
    // 正因如此，&mut型借用也经常被称为“独占指针”，&型借用也经常被称为“共享指针”

    /// # 内存不安全示例
    /// ## 示例一(修改枚举)
    // 加入我们有一个枚举类型

}

// NLL
mod nll {
    // Rust防范内存不安全代码的原则，
    // 如果你对同一块内存存在多个引用，就不要试图对这块内存做修改
    // 如果你需要对一块内存做修改，就不要同时保留多个引用。
    // 只要保证了这个原则，我们就可以保证内存安全

    // 这个原则是没问题的，但是，初始的实现版本有一个ie主要问题，
    // 那就是它让借用指针的生命周期规则和普通对象的生命周期规则一样，是按照作用域来确定的。
    // 所有的变量、借用的生命周期，就是从它声明开始，到整个语句块结束。
    // 这个策略实现起来非常简单，但它可能过度保守了，在某些情况下，现在了程序员的发挥。
    // 因此Rust核心组引入了Non Lexical Lifetime,用更精细的手段调节借用真正起作用的范围，这就是NLL


    /// ## NLL希望解决的问题
    /// 首先来看几个示例

    use std::ascii::AsciiExt;

    fn foo() -> Vec<char> {
        let mut data = vec!['a', 'b', 'c']; // 'scope
        let slice = &mut data[..];//<------ 'lifetime
        capitalize(slice);                            //   |
        // ^----------------------------              //   |
        data.push('d'); //ERROR            //   |
        data.push('e'); //ERROR            //   |
        data.push('f'); //ERROR            //   |
        data                                          //   |
        //<------------------------------------------------|
    }

    fn foo_copy() -> Vec<char> {
        let mut data = vec!['a', 'b', 'c']; // 'scope
        {
            let slice = &mut data[..];//<------ 'lifetime
            capitalize(slice);                            //   |
        } //如果不加{}，在以前版本中会报错
        // ^----------------------------                  //   |
        data.push('d'); //ERROR                //   |
        data.push('e'); //ERROR                //   |
        data.push('f'); //ERROR                //   |
        data                                               //   |
        //<-----------------------------------------------------|
    }
    // 在这段代码中，我们创建了一个临时变量slice,保存了一个指向data的&mut型引用
    // 然后再调用capitalize函数，就出问题了。
    // error[E0499]: cannot borrow `data` as mutable more than once at a time
    // 这是因为Rust规定:"共享不可变，可变不共享，同时出现两个&mut型借用是违反规则的。
    // 在编译器报错的地方，编译器认为slice依然存在，
    // 然而又使用data去调用fn push(&mut self, value T)方法,必然有会参数一个&mut型借用
    // 这违反了Rust的原则。
    // 这种实现方式意味着每个引用的生命周期都是跟代码块相关联的，
    // 它总是从声明的时候被创建，在退出这个代码块的时候被销毁，因此可以称为Lexical lifetime.

    // 而NLL就是取消这个关联性，引用的生命周期，我们用另外的更智能的方式分析。
    // 有了这个功能，上例中手动加入的代码块就不需要了， 编译器能自动分析出来，
    // slice这个引用在capitalize函数调用后就再也没有使用过了，它的生命周期完全可以就此终止
    // 不会对程序的正确性有任何影响，后面再调用push方法修改数据，其实跟前面的slice并不冲突



    fn capitalize(data: &mut [char]) {
        for c in data {
            c.make_ascii_uppercase();
        }
    }

    #[test]
    fn main() {
        let v = foo();
        println!("{:?}", v);
    }


}


