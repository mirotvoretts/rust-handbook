//! 06 (1x) - Половины и "серединка" среза. Эталонное решение.

pub fn halves(xs: &[i32]) -> (&[i32], &[i32]) {
    xs.split_at(xs.len() / 2)
}

pub fn without_ends(xs: &[i32]) -> &[i32] {
    if xs.len() < 2 {
        &[]
    } else {
        &xs[1..xs.len() - 1]
    }
}
