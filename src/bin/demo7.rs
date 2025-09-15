#[derive(Debug, Clone, Default)]
struct Palyer {
    name: String,
    strength: u32,
    hit_points: u32,
}

fn main() {
    let default_palyer = Palyer::default();
    println!("{:?}", default_palyer);
    let mut clone_palyer = default_palyer.clone();
    println!("{:?}", clone_palyer);

    clone_palyer.name = "张三".to_string();
    clone_palyer.strength = 2;
    clone_palyer.hit_points = 2;
    println!("{:?}", clone_palyer);
}
