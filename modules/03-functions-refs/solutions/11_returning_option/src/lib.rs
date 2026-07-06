//! 11 (2x) - Функции, возвращающие `Option`. Эталонное решение.

pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

pub fn nth(xs: &[i32], i: usize) -> Option<i32> {
    xs.get(i).copied()
}

pub fn find_index(xs: &[i32], target: i32) -> Option<usize> {
    for (i, &x) in xs.iter().enumerate() {
        if x == target {
            return Some(i);
        }
    }
    None
}
