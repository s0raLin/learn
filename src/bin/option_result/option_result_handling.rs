fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let s = if let Some(maybe_string) = maybe_string {
        maybe_string
    } else {
        // if let 的两个分支都必须有值，并且类型一致
        // return 用于直接从函数返回 Err(String)
        return Err(String::from("非字符串"));
    };

    let first_char = if let Some(first_char) = s.chars().next() {
        first_char
    } else {
        return Err(String::from("获取首字母失败"));
    };

    if let Some(digit) = first_char.to_digit(16) {
        Ok(digit)
    } else {
        Err(String::from("转成16进制失败"))
    }
}

fn hex_or_dir1(maybe_string: Option<String>) -> Result<u32, String> {
    let s = match maybe_string {
        Some(s) => s,
        _ => return Err(String::from("got None")),
    };
    let first_char = match s.chars().next() {
        Some(first_char) => first_char,
        _ => return Err(String::from("获取失败")),
    };

    //转16进制
    if let Some(digit) = first_char.to_digit(16) {
        Ok(digit)
    } else {
        Err(String::from("not a hex digit"))
    }
}

fn hex_or_dir2(maybe_string: Option<String>) -> Result<u32, String> {
    let Some(s) = maybe_string else {
        return Err(String::from("取出字符串失败"));
    };
    let Some(first_char) = s.chars().next() else {
        return Err(String::from("取出首字母失败"));
    };
    let Some(digit) = first_char.to_digit(16) else {
        return Err(String::from("16进制转换失败"));
    };
    Ok(digit)
}
fn main() {
    println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));
}
