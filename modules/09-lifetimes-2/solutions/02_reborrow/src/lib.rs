//! 02 (1x) — Reborrow: переиспользование `&mut`. Эталонное решение.

pub fn bump(x: &mut i32) {
    *x += 1;
}

pub fn bump_twice(n: &mut i32) {
    bump(n);
    bump(n);
}

pub fn bump_n(n: &mut i32, times: u32) {
    for _ in 0..times {
        bump(n);
    }
}
