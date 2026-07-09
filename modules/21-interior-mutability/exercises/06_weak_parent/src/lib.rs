//! 06 (2x) - дерево с обратной ссылкой на родителя.
//!
//! Ключевое решение этой задачи - выбрать силу ссылок так, чтобы НЕ возникло цикла:
//! одно направление связи должно владеть (сильная ссылка), обратное - только знать, не
//! владея (слабая ссылка). Определитесь, какое из направлений какое, глядя на типы полей
//! Node, и свяжите узлы соответственно. Чтобы прочитать слабую ссылку, её надо поднять.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Узел дерева. value неизменяемо; parent и children - изменяемые поля (RefCell).
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

/// Создай новый узел-лист: без родителя и без детей.
pub fn leaf(value: i32) -> Rc<Node> {
    todo!()
}

/// Присоедини child к parent так, чтобы:
///   - parent стал одним из владельцев child (child попадает в parent.children),
///   - child знал о parent, но НЕ удерживал его жизнь.
pub fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
    todo!()
}

/// Верни значение родителя, если он ещё жив; если родителя нет или он уже уничтожен - None.
pub fn parent_value(node: &Rc<Node>) -> Option<i32> {
    todo!()
}
