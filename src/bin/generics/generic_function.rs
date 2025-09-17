use std::cmp::Ordering;

// TODO: implement the `min` function used in `main`.
fn min<T: Ord>(t1: T, t2: T) -> T {
    match t1.cmp(&t2) {
        Ordering::Less | Ordering::Equal => t1,
        Ordering::Greater => t2,
    }
}
fn main() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}
