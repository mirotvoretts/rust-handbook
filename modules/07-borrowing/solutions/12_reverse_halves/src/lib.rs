//! 12 (3x) — Два непересекающихся `&mut` через `split_at_mut`. Эталонное решение.

pub fn reverse_halves(xs: &mut [i32]) {
    let mid = xs.len() / 2;
    let (left, right) = xs.split_at_mut(mid);
    left.reverse();
    right.reverse();
}
