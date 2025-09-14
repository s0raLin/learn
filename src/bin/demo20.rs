use std::collections::VecDeque;

fn main() {
    let mut deque = VecDeque::new();
    deque.push_front(1);
    deque.push_front(2);
    assert_eq!(deque.get(0), Some(&1));
    assert_eq!(deque.get(1), Some(&2));

}