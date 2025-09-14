use std::rc::Rc;
use std::cell::RefCell;

type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: NodePtr<T>,
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Drop");
    }
}

fn main() {
    let mut first = Rc::new(RefCell::new(Node{data: 1, next: None}));
    let mut second = Rc::new(RefCell::new(Node{data: 2, next: None}));
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(first);
}