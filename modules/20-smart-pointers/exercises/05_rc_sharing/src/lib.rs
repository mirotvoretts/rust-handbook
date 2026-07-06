//! 05 (2x) - Rc: разделяемое владение и счётчик ссылок.
//!
//! Список на Rc позволяет двум головам делить ОДИН хвост. Rc::clone не копирует
//! данные - лишь увеличивает strong_count. Данные живут, пока счётчик > 0.

use std::rc::Rc;

/// Узел списка с разделяемым хвостом.
pub enum Node {
    Cons(i32, Rc<Node>),
    Nil,
}

use Node::{Cons, Nil};

/// Сумма значений списка. Иди по хвостам через match &**node.
pub fn sum(node: &Rc<Node>) -> i64 {
    todo!("match &**node { Cons(v, tail) => *v as i64 + sum(tail), Nil => 0 }")
}

/// Число сильных ссылок на данный узел.
pub fn strong(node: &Rc<Node>) -> usize {
    todo!("Rc::strong_count(node)")
}

/// Построй общий хвост [2, 3] и две головы, разделяющие его:
///   a = 1 -> хвост,  b = 9 -> хвост.
/// Верни (a, b, tail). Хвост клонируй через Rc::clone в обе головы.
pub fn shared_tail() -> (Rc<Node>, Rc<Node>, Rc<Node>) {
    todo!("tail = Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))); a/b с Rc::clone(&tail)")
}
