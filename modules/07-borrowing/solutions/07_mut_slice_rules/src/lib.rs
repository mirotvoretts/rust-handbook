//! 07 (1x) — Изменение `&mut [i32]` на месте. Эталонное решение.

pub fn swap_ends(xs: &mut [i32]) {
    let n = xs.len();
    if n >= 2 {
        xs.swap(0, n - 1);
    }
}

pub fn negate_all(xs: &mut [i32]) {
    for x in xs.iter_mut() {
        *x = -*x;
    }
}
