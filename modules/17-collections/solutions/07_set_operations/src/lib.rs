//! 07 (2x) - Операции над множествами. Эталонное решение.

use std::collections::HashSet;

/// Общие элементы, отсортированные.
pub fn common(a: &[i32], b: &[i32]) -> Vec<i32> {
    let sa: HashSet<i32> = a.iter().copied().collect();
    let sb: HashSet<i32> = b.iter().copied().collect();
    let mut out: Vec<i32> = sa.intersection(&sb).copied().collect();
    out.sort();
    out
}

/// Элементы только из a (нет в b), отсортированные.
pub fn only_in_first(a: &[i32], b: &[i32]) -> Vec<i32> {
    let sa: HashSet<i32> = a.iter().copied().collect();
    let sb: HashSet<i32> = b.iter().copied().collect();
    let mut out: Vec<i32> = sa.difference(&sb).copied().collect();
    out.sort();
    out
}

/// Все ли элементы a есть в b?
pub fn is_covered(a: &[i32], b: &[i32]) -> bool {
    let sa: HashSet<i32> = a.iter().copied().collect();
    let sb: HashSet<i32> = b.iter().copied().collect();
    sa.is_subset(&sb)
}
