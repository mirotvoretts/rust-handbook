//! 06 (1x) — Изменение `&mut [i32]` на месте. Эталонное решение.

pub fn fill(xs: &mut [i32], value: i32) {
    for x in xs.iter_mut() {
        *x = value;
    }
}

pub fn add_to_each(xs: &mut [i32], delta: i32) {
    for x in xs.iter_mut() {
        *x += delta;
    }
}

pub fn reverse_in_place(xs: &mut [i32]) {
    let n = xs.len();
    for i in 0..n / 2 {
        xs.swap(i, n - 1 - i);
    }
}
