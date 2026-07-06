//! 05 (2x) - Rc: разделяемое владение. Эталонное решение.

use std::rc::Rc;

/// Узел с разделяемым хвостом.
pub enum Node {
    Cons(i32, Rc<Node>),
    Nil,
}

use Node::{Cons, Nil};

/// Сумма значений.
pub fn sum(node: &Rc<Node>) -> i64 {
    match &**node {
        Cons(v, tail) => *v as i64 + sum(tail),
        Nil => 0,
    }
}

/// Число сильных ссылок на узел.
pub fn strong(node: &Rc<Node>) -> usize {
    Rc::strong_count(node)
}

/// Две головы, разделяющие общий хвост [2, 3].
pub fn shared_tail() -> (Rc<Node>, Rc<Node>, Rc<Node>) {
    let tail = Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))));
    let a = Rc::new(Cons(1, Rc::clone(&tail)));
    let b = Rc::new(Cons(9, Rc::clone(&tail)));
    (a, b, tail)
}
