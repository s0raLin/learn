fn fizz_buzz(num: i32) -> String {
    if num%15 == 0 {
        "FizzBuzz".to_string()
    } else if num%5 == 0 {
        "Buzz".to_string()
    } else if num%3 == 0 {
        "Fizz".to_string()
    } else {
        num.to_string()
    }
}

fn main() {
    assert_eq!(fizz_buzz(3), "Fizz");
    assert_eq!(fizz_buzz(5), "Buzz");
    assert_eq!(fizz_buzz(6), "Fizz");
    assert_eq!(fizz_buzz(10), "Buzz");
}