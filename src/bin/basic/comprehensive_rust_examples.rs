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

use std::ops::Deref;


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

    //自动deref的规则是，如果一个类型T可以解引用为U,即T: Deref<U>，则&T可以转换为&U


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

}


