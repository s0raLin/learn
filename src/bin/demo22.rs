use std::collections::HashMap;
use std::hash::Hash;

use std::collections::hash_map::Entry;

/// Counter 统计每个类型为 T 的值出现的次数。
struct Counter<T> {
    values: HashMap<T, u64>,
}

impl<T: Eq + Hash> Counter<T> {
    /// 创建一个新的 Counter。
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// 记录给定值出现了一次。
    fn count(&mut self, value: T) {
        match self.values.entry(value) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }

    /// 返回给定值出现过的次数。
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

fn main() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("saw {} values equal to {}", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));
}
