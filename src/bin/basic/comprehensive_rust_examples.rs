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
     #[test]
     fn demo04() {
         let mut i = 0;
         let p1 = &mut i;
        //  let p2 = &mut i;
    
         *p1 = 2;
    
     }
    // error[E0499]: cannot borrow `i` as mutable more than once at a time
    // 因为p1和p2都是可变借用，它们都指向同一个变量，
    // 而且都有修改权限，这是Rust不允许的情况，因此编译无法通过
    // 正因如此，&mut型借用也经常被称为“独占指针”，&型借用也经常被称为“共享指针”

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
    use std::collections::HashMap;

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


    // 让编译器能够更准确地分析借用指针的生命周期，不要简单地与'scope绑定，
    // 不论是普通还是高阶用户而言，都是一个更合理、更有用的功能

    /// ## NLL的原理
    // NLL的设计目的是让借用的生命周期不要过长，适可而止，避免不必要的编译错误。
    // NLL的实现方法不能是简单地在AST上找引用最后一次最后一次在哪里使用就判断它的生命周期结束了
    //
    /// 示例
    fn baz() {
        let mut data = vec!['a', 'b', 'c'];
        let slice = &mut data[..]; //<-+
        loop {                                 //|
            capitalize(slice);                 //|
            //<----------------------------------+
            // data.push('d');
        }
        data.push('e'); // OK
        data.push('f'); // OK
    }
    //在这个示例中，我们引入了一个循环结构，
    // 如果只是分析AST的结构的话，
    // 很可能会觉得capitalize函数结束后，slice的生命周期就结束了，
    // 因此data.push()的方法调用是合理的。
    // 但是这个结论是错误的，因为这里有一个循环结构，
    // 如果执行了push()方法后，引发了Vec数据结构扩容，它把以前的空间释放掉
    // 申请了新空间，进入下一轮循环的时候，slice就会指向一个非法地址，会出现内存不安全。
    // 因此新版的借用检查器不再基于AST的语句块设计，
    // 而是将AST转换为另外一中中间表达形式MIR 在MIR的基础上做分析
    // 对于复杂一点的程序逻辑，基于AST来做生命周期分析是费力不讨好的事，而MIR则更适合做这种分析
    // 可以通过编译器命令打印出MIR文本格式。
    // rustc --emit=mir test.rs
    // 不过在一般情况下，MIR在编译器内部表现形式是内存中的一组数据结构，
    // 这些数据结构描述的是一个叫作控制流图的概念
    // 就是用图这种数据结构来描述程序的执行流程，每个函数都有一个MIR来描述它。
    // 编译器会分析出来，引用在这个图上的哪些节点是活着的，在哪些节点可以看作死掉了，
    // 相比于以前，一个引用的生命周期直接充满整个语句块，现在的表达方式明显要精细得多，
    // 这样我们就可以保证引用的生命周期不被过分拉长

    //另外需要强调的是
    // * 这个功能只影响静态分析结构，不影响程序执行情况
    // * 以前能编译通过的程序以后依然能编译通过，不影响以前的代码
    // * 它依然保证了安全性，只是将以前过于保守的检查规则适当放宽
    // * 他依赖的依然是静态检查规则，不会设计任何动态检查规则
    // * 它只影响“引用类型”的生命周期，不影响“对象”的生命周期，即维持现有的析构函数调用时机不变
    // * 它不会印象RAII语义(资源获取即初始化)

}

// 内部可变性
mod interior_mutability {
    // Rust的borrow checker的核心思想是"共享不可变，可变不共享"
    // 但是只有这个规则是不够的，在某些情况下，我们的确需要在存在共享的情况下可变。
    // 为了让这种情况是可控安全的，Rust设计了一种”内部可变性“
    // Rust的mut关键字不能在声明类型时使用，智能和变量一起使用。
    // 类型本身不能规定自己是否是可变的，一个变量是否是可变的取决于它的使用环境，而不是它的类型。
    // 可变还是不可变取决于变量的使用方式，叫做承袭可变性，
    // 如果用let val: T 声明，那么val就是不可变的，同时，var内部所有成员也都是不可变的
    // 如果let mut var: T声明，那么var就是可变的，相应的，其内部所有成员也都是可变的
    // 我们不能在类型声明的时候指定可变性，比如在struct中对某部分成员使用mut修饰，这是不合法的。
    // 我们只能在变量声明时指定可变性。
    // 我们也不能针对变量的某一部分成员指定可变性，其他保持不变。
    // 常见的具备内部可变性特点的类型有Cell、RefCell、Mutex、RWLock、Atomic*等
    // 其中Cell和RefCell是只能用在单线程环境中的具备可变性的类型

    use std::{cell::{BorrowError, Cell}, rc::Rc, result};

    /// ## Rc
    #[test]
    fn rc_demo() {
        let r1 = Rc::new(1);
        println!("引用个数: {}", Rc::strong_count(&r1));
        
        let r2 = Rc::clone(&r1);
        println!("引用个数: {}", Rc::strong_count(&r2));
    }
    // Rc是Rust中的引用计数智能指针。
    // 多个Rc指针可以同时指向同一个对象，而且有一个共享引用计数值在记录总共有多少个Rc指针指向这个对象。
    // Rc指针提供的是共享引用，，按道理它没有修改共享数据的能力，但我们用共享引用调用clone方法，引用计数值发生了变化。
    // 这时就是内部可变性。如果没有内部可变性，Rc类型是无法正确实现的。具备内部可变性的类型，最典型的就是Cell.
    
    /// 现在用一个更浅显的例子来演示Cell的能力
    #[test]
    fn cell_demo() {
        use std::cell::Cell;

        let data = Cell::new(100);
        let p = &data;
        data.set(10);
        println!("{}", p.get());

        p.set(20);
        println!("{:?}", data);
    }
    // 编译通过
    // 这里的可变性问题和我们前面见到的情况不一样了。
    // data这个变量绑定没有用mut修饰，p这个指针有也没有用&mut修饰
    // 然而不可变引用竟然可以调用set函数改变了变量的值。
    // 这就是内部可变性，这个类型可以通过共享指针修改它内部的值
    // 我们可以存在多个指向Cell类型的不可变引用，同时我们还能利用不可变引用改变Cell内部的值。
    // 实际上这个类型是完全符合内存安全的。
    // Cell类型把数据包裹在内部，用户无法活动指向内部状态的指针，这意味着每次方法调用都是执行的一次完整的数据移动操作。
    // 每次方法调用后，Cell类型的内部都处于一个正确的状态，我们不可能观察到数据被破坏掉的状态。
    // 多个共享指针指向Cell类型的状态，Cell类似一个壳，它把数据包裹在里面，所有的指针只能指向Cell,不能直接指向数据
    // 修改数据只能通过Cell来完成，用户无法创造一个直接指向数据的指针
    
    // Cell提供了一些公开的API
    // impl<T> Cell<T> {
    //     pub fn get_mut(&mut self)->&mut T {}
    //     pub fn set(&self, val: T) {}
    //     pub fn swap(&self, other: &Self) {}
    //     pub fn replace(&self, val: T) {}
    //     pub fn into_inner(self)->T {}
    // }
    // impl<T: Copy> Cell<T> {
    //     pub fn get(&self)->T {}
    // }
    // get_mut：从&mut Cell<T>类型制造出一个&mut T指针。
    // 因为&mut型指针具有独占性，所以这个函数保证了调用前，
    // 有且仅有一个可写指针指向Cell,调用后有且仅有一个可写的指针指向内部数据
    // 它不会出现制造出多个引用指向内部数据的可能性
    
    // set：可以修改内部数据，它是把内部数据整个替换掉，不存在多个引用指向内部数据的可能性

    // swap：修改内部数据。跟set方法一样，也是把内部数据整体替换掉，但区别在于，它仅要求&引用，不要求&mut引用

    // replace:修改内部数据，跟set一样，也是把内部数据整体替换掉，位于的区别是换出来的数据作为返回值返回了。

    // into_inner：相当于把这个壳剥掉了，它接受的是Self类型，即move语义，
    //原来的Cell类型的变量会被move进这个方法，会把内部数据整体返回出来

    // get：接受的是&self参数，返回的是T类型，它可以保持Cell类型不变情况下返回一个新的T类型变量，因此要求T:Copy约束
    // 每次调用它的时候相当于把内部数据memcpy了一份返回出去

    #[test]
    fn demo() {
        let i = Cell::new(100);
        let j = Cell::new(200);
        
        let result = i.into_inner();
        println!("{}", result);
    }

    // RefCell是另一个提供内部可变性的类型，它提供的方式与Cell类型有点不一样。
    // Cell没办法制造出直接指向内部数据的指针，而RefCell可以。
    // impl<T: ?Sized> RefCell<T> {
    //     pub fn borrow(&self)->Ref<T> {}
    //     pub fn try_borrow(&self)->Result<Ref<T>,BorrowError> {}
    //     pub fn borrow_mut(&self)->RefMut<T> {}
    //     pub fn try_borrow_mut(&self)->Result<RefMut<T>,BorrowMutError> {}
    //     pub fn get_mut(&mut self)->&mut T {}
    // }

    // get_mut方法跟Cell::get_mut方法一样，可以通过&mut self获得&mut T,这个过程是安全的。
    // RefCell最主要的两个方法是borrow和borrow_mut
    // 另外两个try_borrow和try_borrow_mut和上面两个的区别在于错误处理方式不同。
    #[test]
    fn refcell_demo() { 
        use std::cell::RefCell;
        
        let shared_vec = RefCell::new(vec![1,2,3]);
        let shared1 = &shared_vec;
        let shared2 = &shared1;

        shared1.borrow_mut().push(4);
        println!("{:?}", shared_vec.borrow());

        shared2.borrow_mut().push(5);
        println!("{:?}", shared_vec.borrow());
    }

    //我们用RefCell包了一个Vec,并且制造了两个共享引用指针指向同一个RefCell

    // 这时，我们可以通过任意一个共享引用调用borrow_mut方法，获得指向内部数据的可写指针
    // 通过这个指针调用push方法，修改内部数据，
    // 同时，我们也可以通过调用borrow方法获取内部的只读指针，读取Vecd里的值

    // 还有一个小问题，
    // 在函数签名中，borrow方法和borrow_mut方法返回的并不是&T和&mut T
    // 而是Ref<T> 和 RefMut<T> 。它们实际上是一种智能指针，完全可以当作&T和&mut T的等价物
    // 标准库之所以返回这样的类型，是因为它需要这个指针生命周期结束后能做点事情
    // 需要自定义包装一下，加上自定义析构函数。
    
    // 问题来了，如果borrow和borrow_mut方法可以制造出指向内部数据的只读、可读写指针，
    // 那么它是如何保证安全性的呢？答案是，
    // RefCell类型放弃了编译阶段的alias+mutation原则，
    // 但依然会在执行阶段保证alias+mutation原则

    // 示例如下
    #[test]
    fn refcell_demo2() { 
        use std::cell::RefCell;

        let shared_vec = RefCell::new(vec![1,2,3]);
        let shared1 = &shared_vec;
        let shared2 = &shared1;

        let p1 = shared1.borrow();
        let p2 = &p1[0];

        // shared2.try_borrow_mut().push(4);
        println!("{}", p2);
    }
    // 编译通过，执行问题来了，程序出现了panic
    // 原因是，RefCell检测到同时出现了alias和mutation的情况
    // 它为了防止更糟糕的内存不安全状态，直接使用了panic来拒绝程序继续执行
    // 如果我们使用try_borrow方法，就会返回值是Result::Err,
    // 这是另外一种更友好的错误处理风格

    // 那么编译器是怎么检测出问题的呢？
    // 原因是，RefCell内部有一个借用计数器，
    // 调用borrow方法时，计数器里面的”共享引用计数“值就加1,
    // 当这个borrow结束后，会将这个值自动减1
    // 同理，当borrow_mut方法被调用时，它就记录一下当前的可变引用。
    // 如果共享引用和可变引用同时出现，就会报错。

    // 从原理来说，Rust默认的"借用规则检查器"的逻辑非常像一个在编译阶段执行的读写锁
    // 如果同时存在多个"读"锁，是没问题的，如果同时存在"读"和"写"锁，或者同时粗壮你多个"写"锁，就会发生错误
    // RefCell类型并没有打破规则，二十将这个检查逻辑从编译阶段移到了执行阶段
    // RefCell让我们可以通过共享引用&修改内部数据，逃过编译器的静态检查。
    // 但是它依然在兢兢业业地保证"内存安全"。
    // 我们需要的借用指针必须通过它提供的API borrow() borrow_mut()来获得
    // 它实际上是在执行阶段在内部维护了一套"读写锁"检查机制
    // 在执行阶段，RefCell是有少量开销的，它需要维护一个借用计数器来保证内存安全

    // 所以，我们一定不要过于滥用RefCell这样的类型，
    // 如果确实有必要使用，请一定规划好动态借用出来的指针存活时间

    // 为了让存在多个alias共享的变量也能被修改，那我们就需要使用内部可变性。
    // Rust提供了只读引用的类型有&、Rc、Arc等指针，它们可以提供alias。
    // Rust提供了内部可变性的类型有Cell、RefCell、Mutex、RWLock以及Atomic*等系列
    // 如果你需要把一个类型T封装到内部可变性类型中，要怎么选择Cell和RefCell呢？
    // 原则就是，如果你只需要整体性地存入、取出T,那么就选Cell.
    // 如果你需要有个可读写指针指向这个T修改它，那就选RefCell





}

use std::ops::{Deref, IndexMut};


// 解引用
mod deref { 
    // 解引用(Deref)和取引用(Ref)的反操作，
    // 取引用，我们有&、&mut等操作符
    // 解引用，我们有*操作符
    #[test]
    fn main() {
        let v1 = 1;
        let p = &v1; //取引用操作
        let v2 = *p; //解引用操作
        println!("{} {}", v1, v2);
    }
    //比如我们有引用类型p: &i32,那么可以用*符号执行解引用操作。
    // 上述例子中，v1的类型是i32,p的类型是&i32, *p的类型又返回i32

    // 自定义解引用
    // 实现标准库中的std::ops::Deref或std::ops::DerefMut这两个trait
    // pub trait Deref {
    //     type Target: ?Sized;
    //     fn deref(&self) -> &Self::Target;
    // }
    // pub trait DerefMut: Deref {
    //     fn deref_mut(&mut self) -> &mut Self::Target;
    // }


    //这个trait有一个关联类型Target,代表解引用后的目标类型

    // 自动解引用
    // Rust提供了自动解引用机制，在某些场景下隐式/自动帮我们做一些事情
    #[test]
    fn demo() {
        let s = "hello";
        println!("length: {}", s.len());
        println!("length: {}", (&s).len());
        println!("length: {}", (&&&&&&&&&&&&&&&s).len())

    }

    // 编译器帮我们做了隐式的deref调用，
    // 当它找不到这个成员方法时，就会自动尝试使用deref方法后再找该方法
    // 一直循环下去

    //自动解引用的规则是，如果一个类型T可以解引用为U,即T: Deref<U>，则&T可以转换为&U


    /// ## 自动解引用的用处 
    /// 用Rc这个智能指针举例，Rc实现了Deref
    #[test]
    fn demo01() {
        use std::rc::Rc;
        let s = Rc::new(String::from("hello"));
        println!("{:?}", s.bytes());
    }

    // 我们创建了一个指向String类型的Rc指针，并调用了bytes()方法，这里是不是很奇怪？
    // 其实等价于s.deref().bytes()
    // String类型其实也没有bytes()方法，但是String可以继续deref,于是再试试s.deref().bytes()
    // 于是找到了bytes()方法
    // 这就是为什么String需要实现Deref trait,是为了让&String类型的变量可以在必要时自动转换为&str类型
    // 所以String类型的变量可以直接调用str类型的方法

    // 同理，Vec<T> 类型也实现了Deref trait,目标类型是[T], 
    // &Vec<T>类型的变量就可以在必要时自动转换为&[T]数组切片类型


    // 有时候需要手动处理
    // 如果智能指针中的方法与它内部成员的方法冲突了怎么办？
    // 编译器会优先调用当前最匹配的内心，而不会指向自动deref,在这种情况下，我们就需要手动deref
    // 比如，Rc类型和String类型都有clone方法，但是他们执行的任务不同。
    // Rc::clone()做的是把引用计数指针复制一份，把引用计数加1。
    // String::clone()做的是把字符串深复制一份。
    
    #[test]
    fn demo02() {
        use std::rc::Rc;
        let s = Rc::new(Rc::new(String::from("hello")));

        let s1 = s.clone();
        
        let ps1 = (*s).clone();

        let pps1 = (**s).clone(); 
    }

    // 一般情况下，在函数调用时，编译器会帮我们尝试自动解引用，但在某些情况下，编译器不会
    // 以String和&str类型为例
    // #[test]
    // fn demo03() {
    //     let s = String::new();

    //     match &s {
    //         "" => {},
    //         _ => {}
    //     }
    // }
    // 这段代码会编译出错
    // match后面的变量类型是&String,匹配分支的变量类型是&'static str,
    // 这种情况就需要我们手动完成类型转换了。方式如下
    #[test]
    fn demo04() {
    
        let s = String::new();

        // 主动调用deref()达到类型转换的目的
        //需要引入use std::ops::Deref;
        use std::ops::Deref;
        match s.deref() {
            "" => {},
            _ => {}
        }

        // 通过*s运算符，强制调用deref()
        match &*s {
            "" => {},
            _ => {}
        }

        //这个方法调用的是标准库中的std::convert::AsRef方法
        // 这个trait存在于prelude中，无需手动引入
        match s.as_ref() {
            "" => {},
            _ => {}
        }

        //需要引入use std::borrow::Borrow;
        use std::borrow::Borrow;
        match s.borrow() {
            "" => {},
            _ => {}

        }

        // 利用String重载的Index操作
        match &s[..] {
            "" => {},
            _ => {}
        }
    }


    // Cow(Copy-on-write)
    // Cow即写时复制技术。它是一种高效的资源管理手段。
    // 假设我们有一份比较昂贵的资源，
    // 当我们需要复制的是很好，可以采用浅复制的方式，在此基础上修改。
    // 这样就不需要重新克隆一份新的资源。
    // 而如果要修改复制后的值，这时候再进行深复制，在此基础上修改。
    // 因此，它的优点是把克隆这个操作推迟到真正需要复制并写操作时发生

    // 在Rust语境中，因为Copy和Clone有明确的语义之分，一般把Cow解释为Clone-on-write
    // 它对指向的数据可能拥有所有权，或者可能不拥有所有权。

    // 当它只需要对指向的数据进行只读访问时，它就只是一个借用指针;
    // 当它需要写数据功能时，它会先分配内存，执行复制操作，再对自己拥有所有权的内存进行写入操作

    // Cow在标准库中是一个enum
    //它可以是Borrowed或Ownedd状态，
    // 如果是Borrowed状态，可以通过调用to_mut()获取所有权,
    // 在这个过程中，实际上它会分配一块新内存，
    // 并将原来Borrowed状态的数据通过调用to_owned()构造出一个新的拥有所有权的对象
    // 然后对这块拥有所有权的内存执行操作。

    // Cow类型最常见的是和字符串配合使用
    use std::borrow::Cow;
    fn remove_spaces(input: &str)->Cow<'_, str> { 
        if input.contains(' ') {
            let mut buf = String::with_capacity(input.len());

            for c in input.chars() {
                if c != ' ' {
                    buf.push(c);
                }
            }
            return Cow::Owned(buf);
        }
        return Cow::Borrowed(input);
    }

    #[test]
    fn test() {
        let s1 = "mp_spaces_in_string";
        let ret1 = remove_spaces(s1);

        let s2 = "space in string";
        let ret2 = remove_spaces(s2);

        println!("{}\n{}", ret1, ret2);
    }
    // 在这个示例中，我们使用Cow类型最主要的目的是优化执行效率。
    // remove_spaces函数的输入参数是&str类型，
    // 如果输入的参数本身就不包含空格，那么我们就直接返回参数本身，无需分配新内存
    // 如果输入参数包含空格，我们就智能在函数体内创建一个新的String对象，用于存储去除掉空格的结果，然后返回

    // 这样一来，就产生了一个小矛盾，这个函数的返回值类型用&str类型和String类型都不太合适。
    // 如果返回值类型是&str，那么需要新分配内存时，就会出现生命周期编译错误。
    // 因为函数内部新分配的字符串的引用不能在函数调用结束后继续存在
    // 如果返回类型指定为String,那么对于那种不需要对参数做修改的情况，会有一些性能损失。
    // 因为输入参数&str类型转为String类型需要分配新的内存空间，并执行复制，性能开销较大。
    // 这时候使用Cow类型就是不二之选。既能满足编译器的生命周期要求，也避免了无谓的数据复制。
    // Cow就是优秀的零性能损失抽象的设计范例

    // Cow还类型还实现了Deref trait,所以当我们需要调用类型T的成员函数时，可以直接调用，
    // 完全无需考虑后面具体是借用指针还是拥有所有权的指针，所以我们也可以把它当成一种智能指针
    
}

// 内存泄漏
mod memory_leak { 

    //首先我们设计一个Node类型，它包含一个指针，可以指向其他的Node实例
    // struct Node {
    //     next: Box<Node>
    // }
    // fn main() {
    //     let node = Node {next: Box::new(...)}
    // }
    // 到这里写不下去了，Rust中要求，Box指针必须被合理初始化，而初始化Box的是很好又必须先传入一个Node实例
    // 这个Node实例有要求创建一个Box指针。。。

    // 要打破这个循环，我们需要使用"可空的指针",在初始化Node时，指针应该是"空状态"，后面再把它们连接起来
    // 我们把代码改进，为了能修改node的值，还需要使用mut.
    // struct Node {
    //     next: Option<Box<Node>>
    // }
    // fn main() {
    //     let mut node1 = Box::new(Node{next: None});
    //     let mut node2 = Box::new(Node{next: None});

    //     node1.next = Some(node2);
    //     node2.next = Some(node1);
    // }

    // 编译错误"error: use of moved value:  'node2'"
    // 在node1.next = Some(node2)这条语句发生了move语义，从此句后，node2变量的生命周期就已经结束了。
    // 因此后面使用node2的时候就发生了错误，我们需要继续改进，不用node2,换而使用node1.next
    // struct Node {
    //     next: Option<Box<Node>>
    // }
    // fn main() {
    //     let mut node1 = Box::new(Node{next: None});
    //     let mut node2 = Box::new(Node{next: None});

    //     node1.next = Some(node2);
    //     match node1.next {
    //         Some(mut n) => n.next = Some(node1),
    //         Node => {}
    //     }
    // }

    //编译又发生了错误，这是因为在match语句中，我们把node1.next的所有权转移到局部变量n中，这个n实际上就是node2的实例，
    // 在赋值操作n.next = Some(node1)过程中，编译器认为此时node1的一部分被专业出去了，它不能在被用于赋值号右边

    // 这是因为我们选择使用的指针类型不对，Box类型的指针对所管理的内存拥有所有权，之使用Box没有办法构造出一个循环引用的结构出来。
    // 于是我们想到使用Rc指针。同时还用了Drop trait来验证这个对象是否被真正释放

    // use std::rc::Rc;

    // struct Node {
    //     next: Option<Rc<Node>>
    // }
    // impl Drop for Node {
    //     fn drop(&mut self) {
    //         println!("析构函数被调用");
    //     }
    // }
    // fn main() {
    //     let mut node1 = Node{next: None};
    //     let mut node2 = Node{next: None};
    //     let mut node3 = Node{next: None};

    //     node1.next = Some(Rc::new(node2));
    //     node2.next = Some(Rc::new(node3));
    //     node3.next = Some(Rc::new(node1));

    // }
    //编译依然没有通过
    // 我们将原来栈上分配内侧改为堆上分配内存
    
    // use std::{rc::Rc};

    // struct Node {
    //     next: Option<Rc<Node>>
    // }
    // impl Drop for Node {
    //     fn drop(&mut self) {
    //         println!("析构函数被调用");
    //     }
    // }

    // fn main() {
    //     let mut node1 = Rc::new(Node{next: None});
    //     let mut node2 = Rc::new(Node{next: None});
    //     let mut node3 = Rc::new(Node{next: None});

    //     node1.next = Some(node2);
    //     node2.next = Some(node3);
    //     node3.next = Some(node1);
    // }

    // 编译再次不通过
    // Rc类型包含的数据是不可变的，通过Rc指针访问内部数据并做修改是不可行的，必须用RefCell把它们包裹起来。
    use std::{rc::Rc, cell::RefCell};
    struct Node {
        next: Option<Rc<RefCell<Node>>>
    }
    impl Node {
        fn new()->Node {
            Node{next: None}
        }
    }
    impl Drop for Node {
        fn drop(&mut self) {
            println!("析构函数被调用")
        }
    }

    fn alloc_objects() {
        let node1 = Rc::new(RefCell::new(Node::new()));
        let node2 = Rc::new(RefCell::new(Node::new()));
        let node3 = Rc::new(RefCell::new(Node::new()));

        node1.borrow_mut().next = Some(node2.clone());
        node2.borrow_mut().next = Some(node3.clone());
        node3.borrow_mut().next = Some(node1.clone());
    }

    #[test]
    fn main() {
        alloc_objects();
        println!("程序结束")
    }

    // 因为我们使用了RefCell,对Node内部数据的修改不再需要mut关键字，编译通过，
    // 执行发现析构函数没有被调用(没有打印输出)
    // 通过循环引用构造内存泄漏，必须同时满足三个条件：
    // 1。使用引用计数指针
    // 2。存在内部可变性
    // 3。指针所指向的内容本身不是'static的

    // 对于上述例子，想避免内存泄漏，
    // 需要程序员手动把内部某个地方的Rc指针替换为std::rc::Weak(弱引用)来打破循环

}

// Panic
mod panic { 

    #[test]
    // 在Rust中有一类错误叫panic
    fn main() {
        let x: Option<i32> = None;
        x.unwrap();
    }
    // 编译没有错误，指向这段程序，输出为:
    // thread 'panic::main' panicked at src/bin/深入浅出rust.rs:1150:11:
    // called `Option::unwrap()` on a `None` value

    // 这种情况就引发了panic,在这段代码中，我们调用了Option::unwrap()方法
    // 正是这个方法有可能导致panic，

    // 调用Option::unwrap()方法
    // 如果Option内部是Some时，它可以成功地将内部数据move出来返回
    // 如果Option内部的数据是None,它会发生panic,panic如果没有被处理，它会导致整个程序崩溃。

    // 在Rust中，正常的错误处理方式是Result类型，Panicc则是作为一种fail fast机制，处理那种万不得已的情况

    // 比如，上例的unwrap方法，试图把Option<i32>转换为i32类型，当参数为None时，这种转换是没办法做到的，这个时候就智能使用panic.
    // 所以，一般情况下，用户应该使用unwrap_or等不会制造panic的方法

    // panic的原理
    // 在Rust中，Panic的实现机制有两种。
    // unwind 发生在panic时，会一层一层推出函数调用栈，在此过程中，当前栈内局部变量可以正常析构
    // abort 发生在panic时，会直接推出整个程序

    // 默认情况下，编译器会使用unwind方式，
    // 所以发生功能panic时，我们可以通过一层层调用栈找到发生panic的第一现场

    //但在某些嵌入式系统中，unwind根本无法实现，或者占用资源太多，这个时候我们可以选择使用abort方式实现panic

    // 

}

// 协变
mod covariance {
    // Rust的生命周期产生是一种泛型类型参数，
    type StrRef<'a> = &'a str;
    // 这是一个指向字符串的借用指针，
    // 它是一个泛型类型，接收一个泛型参数，之后形成一个完整类型
    // 它和Vec<T>很像，只不过Rust中泛型类型参数既有生命周期，又有普通类型，例如
    fn print_str<'b>(a: StrRef<'b>) {
        println!("{}", a);
    }

    fn main() {
        let s: StrRef<'static> = "hello";
        print_str(&s);
    }
    // print_str接受的参数类型是StrRef<'b>，而实际上传进来的参数类型是StrRef<'static>
    // 这两个类型并不完全一致，因为'b != 'static。但是Rust可以接受。
    // 这个现象在类型系统中就叫做协变和逆变
    // 协变：若T1<:T2时满足C<T1> <: C<T2>，则称C对于参数T是协变的
    // 逆变：若T1<:T2时满足C<T2> <: C<T1>，则称C对于参数T是逆变的
    // 不变：上面两种都不成立

    // let s: &str = "hello";
    //"hello" 是一个字符串字面量，它的类型是&'static str。
    //而s是一个局部变量，它的类型是&'s str 其中泛型参数在源码中省略掉了
    //我们把一个生命周期更长的引用&'static str赋值给一个生命周期更短的引用&'s str,这是合法的
    // 原因在于既然这部被指向的目标在一个更长生命周期内都是合法的，
    // 那么它在一个较短生命周期内也一定是合法的。

    // 所以我们可以说引用类型&对生命周期参数具有协变关系
    // 再举例子
    fn test<'a>(s: &'a &'static str) {
        let local: &'a&'a str = s;
    }
    // 可以看到，&'a&'static可以安全地赋值给&'a&'a str类型.
    // 由于&'static str <: &'a str以及&'a &'static str <: &'a &'a str关系成立
    // 这说明引用类型针对泛型参数T也是具备协变类型的。

    //试试&'a mut T指针
    // fn test01<'a>(s: &'a mut &'static str) {
    //     let local: &'a mut &'a str = s;
    // }
    //编译报错，可见出现了生命周期错误，
    //这说明从&'a mut &'static str到&'a mut &'a str的赋值是不安全的
    //这说明&mut 指针对泛型T参数是不可变的。

    fn test02<'a>(s: Box<&'static str>) {
        let local: Box<&'a str> = s;
    }
    // 这段代码编译通过，说明从Box<&'static str>到Box<'a str>的转换是安全的
    // 所以Box<T>类型针对T参数是具备协变类型的。

    //  再试试fn类型，注意fn类型有两个地方可使用泛型参数，
    // 一个是参数，另一个是返回值
    // fn test_arg<'a>(f: fn(&'a str)) {
    //     let local: fn(&'static str) = f;
    // }
    // fn test_ret<'a>(f: fn()->&'a str) {
    //     let local: fn()->&'static str = f;
    // }
    // test_arg通过编译，test_ret不能，意思是，
    // fn(&'a str)类型可以转换为fn(&'static str) 类型
    // fn()->&'a str 类型不能转换为fn()->&'static str类型

    fn test_ret<'a>(f: fn()->&'a str) {
        f();
    }
    fn demo() {
        fn s() -> &'static str { return "" }

        test_ret(s);
    }

    //上面代码编译通过，这意味着fn()->&'static str类型可以安全地转换为fn()->&'a str类型
    // 那我们可以说，类型fn(T)->U对于参数U具备逆变关系

    //再换成具备内部可变性的类型
    // use std::cell::Cell;
    // fn test04<'a>(s: Cell<&'static str>) {
    //     let local: Cell<&'a str> = s;
    // }
    // 编译器出现类型不匹配错误，这说明Cell<T>针对T参数不具备协变关系
    // 因为如果具备内部可变性的类型还有生命周期协变关系，可以构造出悬空指针的情况
    // 所以就需要编译器提供的UnsafeCell来表达针对类型参数具备不变关系的泛型类型。
}


// 迭代器
mod iterator {
    //Vec::iter()方法创建一个动态数组的迭代器，这但是在源码中并没有这个方法，
    // 这是因为这个方法实际上是slice类型的方法，Vec只是自动deref后调用原生数组的迭代器而已

    use std::vec;

    //但Vec类型本身也是可以用于for循环的
    fn main() {
        let v = vec![1,2,3,4,5];
        for i in v {
            println!("{}", i);
        }
    }
    // 这是因为Vec实现了IntoIterator trait,
    // 标准库中的IntoIterator就是编译器留下的一个ie扩展内置for循环语法的接口
    // 任何自定义类型，只要合理地实现了这个trait,就可以被用在内置的for循环里
    //Vec::drain方法可以从殴打宁古塔数组中把一个范围的数据移除，
    // 返回的还是一个迭代器，我们还可以遍历这个迭代器，使用已经被移除的元素

    #[test]
    fn demo() {
        let mut origin = vec![1,2,3,4,5];

        println!("Removed");
        for i in origin.drain(1..3) {
            println!("{}", i);
        }

        println!("Iter");
        for i in origin.iter() {
            println!("{}", i);
        }
    }
}


// 泛型
mod generics {
    // 有时候，我们需要针对多种类型进行统一抽象，这就是泛型。
    // 泛型可以把类型作为参数，在函数或数据结构中使用。
    //以我们数学的Option类型为例
    // enum Option<T> {
    //     Some(T),
    //     None
    // }
    // 这里的<T>实际上就是声明了一个类型参数，在这个Option内部，Some(T)是一个tuple struct
    // 包含一个元素为T,这个泛型参数类型T,可以在使用时指定具体类型。
    #[test]
    fn main() {
        let x = Some(42);
        // let y = None;
    }
    // 在上述代码中，泛型参数T被具体化成了i32,
    //泛型参数可以有多个也可以有默认值
    struct S<T=i32> {
        data: T,
    }
    fn demo() {
        let v1 = S{data: 100};
        let v2 = S::<bool>{data: true};
        println!("{}, {}", v1.data, v2.data);
    }

    //对于上述泛型参数T,我们可以在使用时不指定类型参数，
    //如果不指定，参数默认为i32,也可以在使用时知道那个为其他类型。
    // 使用不同类型参数将泛型类型具体化后，获得的是完全不同的具体类型。
    //比如Option<i32>和Option<i64>是完全不同的类型，不可通用，也不可互相转换。
    // 当编译器生成代码时，会为每一个不同的泛型参数生成不同的代码。
    //各种自定义复合类型都可以携带任意泛型参数。所有泛型参数必须是真的被使用过的，
    // struct Num<T> {
    //     data: i32,
    // } //这段代码会报错，因为结构体声明了一个泛型参数却没有使用它

    // * 函数中的泛型
    fn compare_option<T>(first: Option<T>, second: Option<T>)->bool {
        match (first, second) {
            (Some(..), Some(..))=>true,
            (None, None)=>true,
            _=>false,
        }
    }
    // 在上面的例子中，函数compare_option有一个泛型参数T,两个形参类型均为Option<T>
    // 这意味着两个参数必须是完全一致的类型，如果我们在参数传入两个不同的Option,会导致编译错误

    // fn demo02() {
    //     println!("{}", compare_option(Some(100_i32), Some(200_f32)));
    // }

    //编译器看到这个函数调用时，会进行类型检查，first的形参类型是Option<T>
    // 实参类型是Option<i32>，second的形参类型是Option<T>,实参类型是Option<f32>
    //这是编译器的类型推导功能会进行一个类似解方程组的操作
    // 由Option<T>==Option<i32>可得T==i32,而由Option<T>==Option<f32>又得T==f32;
    // 这就产生了矛盾，此方程无解，导致编译错误

    // 一般情况下调用泛型参数可以不指定泛型参数类型，编译器可以通过类型推导自动判断
    // 某些时候，如果需要手动指定泛型参数类型，则需要使用func::<Type>的形式
    //泛型参数在很大程度上实现了C++的"函数重载",比如str类型有一个contains方法
    #[test]
    fn demo05() {
        let s = "hello";
        println!("{}", s.contains('h'));
        println!("{}", s.contains("abc"));
        println!("{}", s.contains(&['H'] as &[char]));
        println!("{}", s.contains(|c: char| c.len_utf8() > 2));
    }
    // 不妨看看实现
    //pub fn contains<P: Pattern>(&self, pat: P) -> bool
    // 这个方法有一个泛型参数P,它有一个约束Pattern,
    // 意味着所有实现了Pattern trait的类型都可以作为参数传入contains方法

    //我们还有一种方案，可以把不同的类型统一起来，
    // 那就是enum,通过enum的不同成员携带不同的类型信息，可以做到类似"函数重载"的功能


    // * impl块中的泛型
    // impl有时候也可以使用泛型,在impl<Trait> for <Type>{}这个语法结构中
    // 泛型类型既可以出现在Trait中，也可以出现在Type中
    // 与其他地方的泛型一样，impl块中的泛型需要先声明后使用，
    // 在impl块中出现的泛型参数，需要紧跟在impl关键字后声明
    // 当我们需要为某一组类型统一impl某一个trait时，
    // 有了这个功能，很多时候就没必要单独为每个类型去充分impl了
    // impl <T, U> Into<U> for T 
    //     where U: From<T> {
    //     fn into(self)->U {

    //     }
    // }
    // 这段代码中，impl关键字后声明了两个泛型参数T、U，后面会使用它们
    // 这跟类型、函数中的泛型参数规则一样，先声明、后使用

    // 标准库中的Into和From是一对功能互逆的trait,如果A::Into<B>，意味着B::From<A>
    // 因此标准库中这段代码，意思是针对所有类型T,只要满足U::From<T>，那么就针对类型impl Into<U>
    // 有这样的一个impl块后，如果想为自己的两个类型提供互相转换的功能，
    // 就只需impl From这一个trait就可以了

    // * 泛型参数约束
    //Rust的泛型类型检查是当场完成的，它会分析泛型函数的时候当场检查类型的合法性
    // 它要求用户提供合理的泛型约束，在Rust中，trait可以用于作为泛型约束
    // fn max<T>(a: T, b: T)->T {
    //     if a < b {
    //         b
    //     } else {
    //         a
    //     }
    // }
    // #[test]
    // fn demo06() {
    //     println!("{}", max(100, 200));
    //     // println!("{}", max(1.5, 2.5));
    // }
    //编译报错
    //binary operation `<` cannot be applied to type `T`r
    // 由于T没有任何约束，因此编译器认为a<b这个表达式是不合理的，
    // 因为它只用于支持比较运算符的类型，
    //在Rust中，只有impl了这个PartialOrd类型，才能支持比较运算符
    // 修复方案是给T加泛型约束
     fn max<T: PartialOrd>(a: T, b: T)->T {
        if a < b {
            b
        } else {
            a
        }
    }
    #[test]
    fn demo06() {
        println!("{}", max(100, 200));
    }
    //泛型参数声明的时候使用:指定
    // 使用where子句指定
    // fn min<T, U>(a: T, b: T)->T
    //     where T: PartialOrd {
    //     if a < b {
    //         a
    //     } else {
    //         b
    //     }
    //}
    //这两个写法达到的目的是一样的，但是在某些情况下where子句会更方便

    // *关联类型
    // trait还也包含类型，比如Iterator trait
    // pub trait Iterator {
    //     type Item;
    //     ...
    // }
    // 这样在trait中声明的类型叫作关联类型。
    // 关联类型同样是这个trait的”泛型参数。“
    // 只有指定了所有泛型参数和关联类型，这个trait才能真正具体化
    use std::iter::Iterator;
    use std::fmt::Debug;
    fn use_iter<ITEM, ITER>(mut iter: ITER) 
        where ITER: Iterator<Item=ITEM>, ITEM: Debug {
            while let Some(i) = iter.next() {
                println!("{:?}", i);
            }
    }
    #[test]
    fn demo07() {
        let v = vec![1, 2, 3, 4, 5];
        use_iter(v.iter());
    }
    fn use_iter2<ITER>(mut iter: ITER) 
        where ITER: Iterator<Item=i32> {
            while let Some(i) = iter.next() {
                println!("{}", i);
            }
    }
    // 这个版本的写法相对于上版本来说，泛型参数明显简化了，我们只需要一个泛型参数
    // 在泛型约束条件中，可以写ITER符合Iterator约束。

}

mod closure {
    #[test]
    fn main() {
        let add1 = |a, b| -> i32 {return a+b};
        let add2 = |a, b|{ a + b};
        let add3 = |a,b| a+b;
        let x = add1(1, 2);
        let y = add2(1, 2);
        let z = add3(1, 2);
        println!("result: {}",x);
    }

}



// 容器
mod eontainer {
    // ? Vec -> 可变长数组，连续存储
    // ? VecDeque -> 双向队列，适用于从头部和尾部插入和删除元素
    // ? LinkedList -> 链表，非连续存储
    // ? HashMap -> 基于Hash算法存储一系列键值对
    // ? BTreeMap -> 基于B树 存储一系列键值对
    // ? HashSet -> 基于哈希算法的集合，相当于没有值的HashMap
    // ? BTreeSet -> 基于B树的集合，相当于没有值的BTreeMap
    // ? BinaryHeap -> 基于二叉堆实现的优先级队列

    // * 1. Vec
    // Vec是最常用的容器，就是一个可以自动扩容的动态数组
    // 重载了Index运算符，可以通过[]访问对应下标元素（内部成员）
    // 重载了Deref/DerefMut运算符，可以自动解引用为数组切片
    
    // ?常用方法
    mod vec_example {
        mod mod1 {
            use std::vec;

            #[test]
            fn main() {
                // ?常见的几种Vec构造方式
                // 1. new方法与default()方法一样，构造一个空的Vec
                let v1 = Vec::<i32>::new();
                // 2. with_capacity() 可以预先分配一个较大的空间，避免插入数据时动态扩容
                let v2 = Vec::<String>::with_capacity(1000);
                // 3. 利用宏来初始化，语法和数组初始化类似
                let v3 = vec![1, 2, 3];
                
                // ?插入数据
                let mut v4 = Vec::<i32>::new();
                v4.push(1);
                v4.extend_from_slice(&[10, 20, 30, 40, 50]);
                v4.insert(2, 100);
                println!("{:?}", v4);

                // ?访问元素
                // 调用IndexMut运算符，可以写数据
                v4[5] = 5;
                let i = v4[5];
                println!("{}", i);
                // Index运算符直接访问，如果越界会造成panic,而get方法不会，因为它返回一个Option<T>
                if let Some(i) = v4.get(6) {
                    println!("{}", i);
                }

                // Index运算符支持各种Range作为索引
                let slice = &v4[4..];
                println!("{:?}", slice);
            }
        }
        // 一个Vec中能存储的元素个数最多为std::usize::MAX个，超过了会发生panic.
        // 因为它会记录元素个数，用的是usize类型。
        // 如果我们指定元素的类型是0大小的类型，那这个Vec根本不需要在堆上分配空间
        // Vec中存在一个指向堆的指针，它永远是非空状态，
        // 编译器可以据此做优化使得size_of::<Option<Vec<T>>>() == size_of::<Vec<T>>
        // 示例如下
        mod mod2 {
            struct ZeroSized();

            #[test]
            fn main() {
                let mut v = Vec::<ZeroSized>::new();
                println!("capacity:{} length:{}", v.capacity(), v.len());

                v.push(ZeroSized());
                v.push(ZeroSized());
                println!("capacity:{} length:{}", v.capacity(), v.len());

                // p永远指向align_of::<ZeroSized>() 不需要调用allocator
                let p = v.as_ptr();
                println!("pointer:{:p}", p);

                let size1 = std::mem::size_of::<Vec<i32>>();
                let size2 = std::mem::size_of::<Option<Vec<i32>>>();
                println!("size of Vec:{} size of option vec:{}", size1, size2);
            }
        }
        // 编译执行结果为
        // *capacity:18446744073709551615 length:0
        // *capacity:18446744073709551615 length:2
        // *pointer:0x1
        // *size of Vec:24 size of option vec:24
    }

    // ?VecDeque
    mod vec_deque_example {
        use std::collections::VecDeque;

        // *VecDeque是一个双向队列，在它的头部或尾部指向添加或删除操作效率很高
        // *用法和Vec非常类似，主要多了pop_front() push_front()等方法
        #[test]
        fn main() {
            let mut queue = VecDeque::<i32>::with_capacity(64);
            //?向尾部按顺序一个一个插入数据
            for i in 1..10 {
                queue.push_back(i);
            }

            //?从头部按顺序一个一个取出
            while let Some(i) = queue.pop_front() {
                println!("{}", i);
            }

        }
    }

    //?HashMap
    mod hashmap_example {
        // HashMap<K, V, S>是基于hash算法的存储一组键值对(key-value-pair)的容器，
        // 其中，泛型参数K是键的类型，V是值类型，S是哈希算法的类型。
        // S这个泛型参数有一个默认值，平时我们使用的时候不需要手动指定，如果有特殊需要，则可以自定义哈希算法。
        // hash算法的关键是，将记录的存储地址和key之间建立一个确定的对应关系。
        // 这样，当想查找某条记录时，我们根据记录的key,通过一次函数计算，就可以得到它的存储地址
        // 进而快速判断这条记录是否存在，存储在哪里
        // 因此 Rust的HashMap要求，key要满足Eq+Hash的约束，
        // Eq trait代表这个类型可以作相等比较，并且一定能够满足三个性质
        // * 自反性，对任意a,满足a==a;
        // * 对称性，如果 a==b成立，则 b==a 成立
        // * 传递性，如果 a==b 且 b==c 成立，则 a==c 成立
        // 而Hash trait更重要
        // Hash方法基本上是重复性的代码，因此编译器提供了自动derive实现
        #[derive(Hash)]
        struct Person {
            first_name: String,
            last_name: String,
        }
        // 一个完整使用HashMap的示例如下
        mod mod1 {
            use std::collections::HashMap;

            #[derive(Hash, Eq, PartialEq, Debug)]
            struct Person {
                first_name: String,
                last_name: String,
            }
            impl Person {
                fn new(first: &str, last: &str)->Self {
                    Person {
                        first_name: first.to_string(),
                        last_name: last.to_string(),
                    }
                }
            }

            #[test]
            fn main() {
                let mut book =  HashMap::new();
                book.insert(Person::new("John", "Smith"), "521-8926");
                book.insert(Person::new("Sandra", "Dee"), "521-9655");
                book.insert(Person::new("Ted", "Baker"), "418-4165");

                let p = Person::new("John", "Smith");

                // 查找key对应的值
                if let Some(phone) = book.get(&p) {
                    println!("手机号: {}", phone);
                }

                // 删除
                book.remove(&p);

                // 查询是否存在
                println!("查找键: {}", book.contains_key(&p));
            }
        }
        // ?HashMap的查找、插入、删除操作的平均时间复杂度都是O(1)
        
        // ?HashMap还设计了一种叫entry的系列API
        mod mod2 {
            // 考虑这样的场景，
            // 我需要先查看某个key是否存在，然后再做插入或删除操作，
            // 这种时候，如果我们用传统API,需要执行两遍查找操作
            // * if map.contains_key(key) { //执行了一遍hash查找操作
            // *    map.insert(key, value); // 又执行了一遍hash查找操作
            // * }
            // 如果我们用entry API,则可以提高效率，而且代码也更流畅
            // *map.entry(key).or_insert(value);
            // HashMap也实现了迭代器，使我们可以直接变量整个容器
            // HashMap中，key存储的位置跟它本身的值密切相关，
            // 如果key本身变了，那么它存放的位置也需要相应变化
            // 所以 HashMap设计的各种api中，执行key的借用一般是只读借用，防止用户修改
            // 但是，只读借用并不能保证它不被修改，只读借用依然可以改变具备内部可变性特点的类型。
            // !下面示例演示，如果动态修改HashMap中的key值，会出现什么后果
            mod mod3 {
                use std::{cell::Cell, collections::HashMap, hash::{Hash, Hasher}};

                #[derive(Eq, PartialEq)]
                struct BadKey {
                    value: Cell<i32>,
                }
                impl BadKey {
                    fn new(v: i32)->Self {
                        BadKey { value: Cell::new(v) }
                    }
                }
                impl Hash for BadKey {
                    fn hash<H: Hasher>(&self, state: &mut H) {
                        self.value.get().hash(state);
                    }
                }

                #[test]
                fn main() {
                    let mut map = HashMap::new();
                    map.insert(BadKey::new(1), 100);
                    map.insert(BadKey::new(2), 200);

                    for key in map.keys() {
                        key.value.set(key.value.get()*2);
                    }

                    println!("Find key 1:{:?}", map.get(&BadKey::new(1)));
                    println!("Find key 2:{:?}", map.get(&BadKey::new(2)));
                    println!("Find key 3:{:?}", map.get(&BadKey::new(4)));
                }
                // 在上面示例中，我们设计了一个具备内部可变性的类型作为key,然后直接在容器内将它的值改变
                // 接下来做查找，可以看到我们再也找不到这几个key了，无论是用修改前的key,还是修改后的key
                // 这种错误属于逻辑错误，是编译器无法静态检查出来的，
                // 所有关联容器HashMap、HashSet、BTreeMap、BTreeSet都存在这样的情况，
                


            }

            // ?标准库的HashSet类型和HashSet非常类似，主要区别是它之一key没有value

            
        }
    }

    //?BTreeMap
    // BTreeMap<K, V>是基于B树数据结构的存储一组键值对的容器
    // 它跟HashMap的用途类似，但是内部树机制不同，
    // B树的每个节点包含多个连续存储的元素，以及多个子节点
    // BTreeMap对key的要求是满足Ord约束，即具备全序特征            \
    mod btreemap_example {
        mod mod1 {
            use std::collections::BTreeMap;
            #[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]            
            struct Person {
                first_name: String,
                last_name: String,
            }
                    
            impl Person {             
                fn new(first: &str, last: &str)->Self {
                    Person {
                        first_name: first.to_string(),
                        last_name: last.to_string(),
                    }
                }
            }
            #[test]
            fn main() {
                let mut book = BTreeMap::new();

                book.insert(Person::new("John", "Smith"), "521-8976");
                book.insert(Person::new("Sandra", "Dee"), "521-9655");

                let p = Person::new("John", "Smith");

                        // 查找键对应的值
                if let Some(phone) = book.get(&p) {
                    println!("Phone number found: {}", phone);
                }

                        // 删除
                book.remove(&p);

                        // 查询是否存在
                println!("Find key: {}", book.contains_key(&p));

            }
        }

        mod mod2 {
            // BTreeMap也实现了entry API,BTreeMap也实现了迭代器，同样可以直接遍历
            // 但是HashMap在遍历时，是不保证遍历结果顺序的，而BTreeMap自动报数据排好了

            // BTreeMap比HashMap多的一项功能是，它不仅可以查询但个key的结果，还可以查询一个区间的结果

            use std::collections::BTreeMap;

            #[test]
            fn main() {
                let mut map = BTreeMap::new();
                map.insert(3, "a");
                map.insert(5, "b");
                map.insert(8, "c");

                for (k, v) in map.range(2..6) {
                    println!("{}:{}",k,v);
                }
            }
            // 当然，我们还可以使用其他Range类型，如RangeInclusive等
        }
    }

    // BTreeSet类型跟BTreeMap非常类似，主要区别在于它只有key没有value
    // struct BTreeSet<T> {
    //     map: BTreeMap<T. ()>
    // }
    
    // ?迭代器
    // 迭代器是指实现了Iterator trait的类型，
    // * trait Iterator {
    // *     type Item;
    // *     fn next(&mut self)->Option<Self::Item>;
    // * }
    // 它主要的一个方法就是next(),返回一个Option<Item>。
    // 一般情况返回Some(Item),如果迭代完成，就返回None
    // ?实现迭代器
    // 假设我们的目标是，这个迭代器会生产一个从1到100的序列，我们需要创建一个类型，这里使用struct
    // 它要实现Iterator trait, 注意到：
    // 每次调用next方法时，它都返回不同的值，所以它一定要有一个成员，能记录上次返回的是什么
    mod iterator_exmaple {
        mod mod1 {
            struct Seq {
                current: i32,
            }
            impl Seq {
                fn new()->Self {
                    Seq {current: 0}
                }
            }
            impl Iterator for Seq {
                type Item = i32;

                fn next(&mut self)->Option<i32> {
                    if self.current < 100 {
                        self.current+=1;
                        Some(self.current)
                    } else {
                        None
                    }
                }
            }

            #[test]
            fn main() {
                let mut seq = Seq::new();
                while let Some(i) = seq.next() {
                    println!("{}", i);
                }
            }
        }

        //?迭代器组合
        //Rust标准库有一个命名规范，从容器创造出迭代器一般有三种方法
        // * iter() 创造出一个Item是&T类型的迭代器
        // * iter_mut() 创造出一个Item是&mut T类型的迭代器
        // * into_iter() 创造出一个Item是T类型的迭代器

        // 示例如下
        mod mod2 {
            #[test]
            fn main() {
                let v = vec![1,2,3,4,5];
                let mut iter = v.iter();
                while let Some(i) = iter.next() {
                    println!("{}",i);
                }
            }
        }
        // 如果迭代器就这么简单，那么它的用处基本就不大了，
        // Rust的迭代器有一个特点，就是它是可组合的。
        // Iterator trait内有一大堆方法，比如nth、map、filter、skip_while、take等等
        // 这些方法都有默认实现，它们可以统一为适配器，
        // 它们有个共性，返回的是一个具体类型，而这个类型本身也实现了Iterator trait
        // 这意味着我们调用这些方法可以从一个迭代器创造出一个新的迭代器
        mod mod3 {
            #[test]
            fn main() {
                let v = vec![1,2,3,4,5,6];
                let mut iter = v.iter()
                    .take(5)
                    .filter(|&x| x%2==0)
                    .map(|&x| x*x)
                    .enumerate();
                while let Some((i, v)) = iter.next() {
                    println!("{} {}", i, v);
                }
            }
        }
        // 构造一个迭代器本身，是代价很小的行为，因为它只初始化了一个对象，并不真正产生和消费数据
        // 不论迭代器内部嵌套了多少层，最终消费数据还是要调用next()方法实现的，这个特点也被称为惰性求值。
        // 如果用户写了下面的代码，
        // * let v = vec![1,2,3,4,5];
        // * v.iter().map(|x|, println!("{}",x));
        // 实际上什么也没做，因为map方法只是把前面的迭代器包装，构造一个新的迭代器，没有真正读取容器内的数据
    }

    // ?for循环
    // 在前面的示例中，我们都是手工调用迭代器的next()方法，然后使用while let语法来做循环
    // 实际上，Rust里更简洁，跟自然地使用迭代器的方式是for循环
    // 本质上for循环是专门为迭代器设计的语法糖，
    // * for循环可以对针对数组切片、字符串、Range、Vec、LinkedList、HashMap、BTreeMap等
    // *所有具有迭代器的类型执行循环

    mod for_exmaple {
        use std::collections::HashMap;
        

        #[test]
        fn main() {
            let v = vec![1,2,3,4,5,6];
            for i in v {
                println!("{}", i);
            }

            let map: HashMap<i32, char> = 
                [(1, 'a'),(2, 'b'),(3, 'c'), (4, 'd')].iter().cloned().collect();
            for (k, v) in &map {
                println!("{} : {}", k, v);
            }
        }
        // 只要实现了IntoIterator,那么调用into_iter()方法就可以得到对应的迭代器。
        // 这个into_iter()方法的receiver(接收者)是self而不是&self,执行的是move语义
        // 这么做，可以同时支持Item类型为T、&T、&mut,用户有选择的权利。

        // Rust中的for <item> in <container> {<body>} 语法结构就是一个语法糖，
        // 这个语法的原理是调用<container>.into_iter()获得迭代器，
        // 然后不断循环调用迭代器的next()方法，
        // 将返回值解包，赋值给<item>，然后调用<body>语句块
        // Rust中的IntoIterator trait实际上就是for语法的扩展接口
        // *如果我们需要让各种自定义容器也能在for循环中使用，
        // *那就可以借鉴标准库中的写法，自行实现这个trait

    

    }

}

// ?生成器
// 在rust中，协程是编写高性能异步程序的关键设施，生成器是协程的基础
// 生成器的语法就像闭包，但和闭包有一个区别，即yield关键字。
// 当闭包中有yield关键字时，它就不是一个闭包，而是一个生成器
