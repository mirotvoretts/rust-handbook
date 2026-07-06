//! 03 (1x) - Рекурсивный тип: cons-список на Box.
//!
//! Без Box вариант Cons(i32, List) имел бы бесконечный размер (E0072).
//! Box даёт косвенность фиксированного размера. Обходи список через match.

/// Односвязный список: узел (значение + хвост) или конец.
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

/// Построй список из среза, сохраняя порядок: [1,2,3] -> Cons(1, Cons(2, Cons(3, Nil))).
/// Идёшь с конца: хвост для последнего элемента - Nil.
pub fn from_slice(xs: &[i32]) -> List {
    todo!("сложи с конца: xs.iter().rev().fold(Nil, |tail, &x| Cons(x, Box::new(tail)))")
}

/// Длина списка (число узлов Cons).
pub fn len(list: &List) -> usize {
    todo!("match list { Cons(_, tail) => 1 + len(tail), Nil => 0 }")
}

/// Сумма значений.
pub fn sum(list: &List) -> i64 {
    todo!("match list { Cons(v, tail) => *v as i64 + sum(tail), Nil => 0 }")
}
