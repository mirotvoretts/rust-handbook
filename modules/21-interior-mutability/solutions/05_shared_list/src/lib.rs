//! 05 (2x) - список Rc<RefCell<Node>>. Эталонное решение.

use std::cell::RefCell;
use std::rc::Rc;

/// Ссылка на следующий узел.
pub type Link = Option<Rc<RefCell<Node>>>;

/// Узел односвязного списка.
pub struct Node {
    pub value: i32,
    pub next: Link,
}

/// Собрать список из среза: xs[0] - голова.
pub fn from_slice(xs: &[i32]) -> Link {
    let mut next: Link = None;
    for &v in xs.iter().rev() {
        next = Some(Rc::new(RefCell::new(Node { value: v, next })));
    }
    next
}

/// Сумма значений.
pub fn sum(head: &Link) -> i64 {
    let mut cur = head.clone();
    let mut total = 0i64;
    while let Some(node) = cur {
        total += node.borrow().value as i64;
        cur = node.borrow().next.clone();
    }
    total
}

/// Прибавить delta ко всем значениям на месте.
pub fn add_all(head: &Link, delta: i32) {
    let mut cur = head.clone();
    while let Some(node) = cur {
        node.borrow_mut().value += delta;
        cur = node.borrow().next.clone();
    }
}
