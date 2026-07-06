//! 07 (2x) - Операции над множествами.
//!
//! У HashSet есть готовые union/intersection/difference (возвращают итераторы) и
//! предикаты is_subset/is_disjoint. Результаты собирайте отсортированными - тестам
//! нужен детерминизм (соберите в Vec и sort()).

use std::collections::HashSet;

/// Общие элементы, отсортированные.
pub fn common(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!("два HashSet::from_iter, intersection, collect, sort")
}

/// Элементы только из a (нет в b), отсортированные.
pub fn only_in_first(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!("difference")
}

/// Все ли элементы a есть в b?
pub fn is_covered(a: &[i32], b: &[i32]) -> bool {
    todo!("is_subset")
}
