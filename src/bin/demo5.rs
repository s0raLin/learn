trait Animal {
    fn leg_count(&self) -> u32;
}

///Pet 也是一个 trait，并且写作 trait Pet: Animal，意味着 Pet 继承自 Animal：
///
/// 所有实现 Pet 的类型也必须实现 Animal。
///
/// 另外，Pet 自己定义了一个 name 方法，用来获取宠物的名字。
trait Pet: Animal {
    fn name(&self) -> String;
}

struct Dog(String);

impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}

fn main() {
    let puppy = Dog(String::from("Rex"));
    println!(" has {} legs", puppy.leg_count());
}
