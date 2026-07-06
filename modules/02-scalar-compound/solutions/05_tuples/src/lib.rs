//! 05 (1x) - Кортежи. Эталонное решение.

pub fn divmod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

pub fn min_max(a: i32, b: i32, c: i32) -> (i32, i32) {
    let lo = a.min(b).min(c);
    let hi = a.max(b).max(c);
    (lo, hi)
}

pub fn swap_pair(p: (i32, i32)) -> (i32, i32) {
    let (x, y) = p;
    (y, x)
}
