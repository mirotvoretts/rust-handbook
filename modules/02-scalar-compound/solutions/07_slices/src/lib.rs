//! 07 (1x) - Срезы `&[i32]`. Эталонное решение.

pub fn sum_slice(xs: &[i32]) -> i32 {
    let mut acc = 0;
    for &x in xs {
        acc += x;
    }
    acc
}

pub fn max_slice(xs: &[i32]) -> Option<i32> {
    let mut best: Option<i32> = None;
    for &x in xs {
        best = Some(match best {
            Some(b) if b >= x => b,
            _ => x,
        });
    }
    best
}

pub fn first_last(xs: &[i32]) -> Option<(i32, i32)> {
    match (xs.first(), xs.last()) {
        (Some(&f), Some(&l)) => Some((f, l)),
        _ => None,
    }
}
