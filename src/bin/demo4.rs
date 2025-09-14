
/// Trait |	用途
/// ---  | ---
/// Debug |	调试输出
/// Clone |	克隆对象，生成 .clone() 方法
/// Copy |	简单拷贝（仅用于小类型，如整数）
/// PartialEq |	比较相等（== / !=）
/// Eq |	完全相等
/// Default |	提供默认值（::default()）
/// Hash |	支持哈希表（HashMap）
/// PartialOrd / Ord |	支持排序
/// Debug 是 Rust 标准库提供的一个 trait，用于 格式化输出调试信息 </br>
/// 实现了 Debug 的类型可以用 {:?} 或 {:#?} 打印
#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    // 没有接收者，这是一个静态方法
    fn new(name: &str) -> Self {
        Self { name: String::from(name), laps: Vec::new() }
    }

    // 对 self 提供独占的可变借用访问
    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // 对 self 提供共享的只读借用访问
    fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() { //迭代器遍历元素并包装成(key, value)
            println!("Lap {idx}: {lap} sec");
        }
    }

    // 对 self 提供独占的所有权
    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
    // race.add_lap(42); // 这里不能再使用 race，因为它的所有权已经被 finish() 移走
}
