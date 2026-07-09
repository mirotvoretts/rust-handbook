//! 08 (3x) - двусторонняя связь без утечки. Эталонное решение.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Узел двусвязной пары.
pub struct Node {
    pub value: i32,
    pub next: RefCell<Option<Rc<Node>>>,
    pub prev: RefCell<Option<Weak<Node>>>,
}

/// Одиночный узел.
pub fn node(value: i32) -> Rc<Node> {
    Rc::new(Node {
        value,
        next: RefCell::new(None),
        prev: RefCell::new(None),
    })
}

/// Связать a -> b: сильно вперёд, слабо назад.
pub fn pair(a: i32, b: i32) -> (Rc<Node>, Rc<Node>) {
    let a = node(a);
    let b = node(b);
    *a.next.borrow_mut() = Some(Rc::clone(&b));
    *b.prev.borrow_mut() = Some(Rc::downgrade(&a));
    (a, b)
}

/// Значение следующего узла.
pub fn next_value(n: &Rc<Node>) -> Option<i32> {
    n.next.borrow().as_ref().map(|rc| rc.value)
}

/// Значение предыдущего узла, если он жив.
pub fn prev_value(n: &Rc<Node>) -> Option<i32> {
    n.prev
        .borrow()
        .as_ref()
        .and_then(|w| w.upgrade())
        .map(|rc| rc.value)
}
