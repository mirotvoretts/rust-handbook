//! 10 (2x) - Заём среза, владеющий результат. Эталонное решение.

pub fn max_pair_sum(xs: &[i32]) -> Option<i32> {
    if xs.len() < 2 {
        return None;
    }
    let mut best = xs[0] + xs[1];
    for i in 2..xs.len() {
        let s = xs[i - 1] + xs[i];
        if s > best {
            best = s;
        }
    }
    Some(best)
}
