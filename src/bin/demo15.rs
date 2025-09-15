fn main() {
    let name = "Löwe 老虎 Léopard Gepardi";
    let mut option = name.find("é");
    println!("查找返回： {option:?}");
    assert_eq!(option.unwrap(), 14);
    option = name.find('Z');

    println!("查找返回： {option:?}");
    assert_eq!(option.unwrap(), 0);
}
