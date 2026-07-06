//! 03 (1x) - Рекурсивный тип: cons-список на Box. Эталонное решение.

/// Односвязный список.
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

/// Построение из среза с сохранением порядка.
pub fn from_slice(xs: &[i32]) -> List {
    xs.iter().rev().fold(Nil, |tail, &x| Cons(x, Box::new(tail)))
}

/// Число узлов.
pub fn len(list: &List) -> usize {
    match list {
        Cons(_, tail) => 1 + len(tail),
        Nil => 0,
    }
}

/// Сумма значений.
pub fn sum(list: &List) -> i64 {
    match list {
        Cons(v, tail) => *v as i64 + sum(tail),
        Nil => 0,
    }
}
