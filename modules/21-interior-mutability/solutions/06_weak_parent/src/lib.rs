//! 06 (2x) - дерево с родителем через Weak. Эталонное решение.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Узел дерева.
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

/// Новый узел-лист.
pub fn leaf(value: i32) -> Rc<Node> {
    Rc::new(Node {
        value,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    })
}

/// Присоединить child к parent: сильно вниз, слабо вверх.
pub fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
    parent.children.borrow_mut().push(Rc::clone(child));
    *child.parent.borrow_mut() = Rc::downgrade(parent);
}

/// Значение родителя, если он ещё жив.
pub fn parent_value(node: &Rc<Node>) -> Option<i32> {
    node.parent.borrow().upgrade().map(|p| p.value)
}
