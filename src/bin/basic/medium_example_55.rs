use std::rc::Rc;

/**
 * Rc<T>
 *
 * Rc<T> 是一个引用计数智能指针，Rc<T> 允许多个指针同时指向一个值。
 *
 * Rc<T> 允许数据 interior mutability，
 *
 *
 *
 */
fn main() {
    let x = Rc::new(45);

    let y1 = x.clone(); //增加强引用计数
    let y2 = x.clone(); //增加强引用计数

    println!("{}", Rc::strong_count(&x));

    let w = Rc::downgrade(&x); // 增加弱引用计数
    // 共享的指针没有所有权，因此被称为弱引用

    println!("{}", Rc::weak_count(&x));

    let y3 = &*x; //不增加计数
    println!("{}", 100 - *x);

    println!("{}", Rc::strong_count(&x))
}
