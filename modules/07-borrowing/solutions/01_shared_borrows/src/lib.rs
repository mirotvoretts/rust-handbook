//! 01 (0x) — Много общих заёмов `&` одновременно. Эталонное решение.

pub fn total(xs: &Vec<i32>) -> i32 {
    xs.iter().sum()
}

pub fn count_positive(xs: &Vec<i32>) -> usize {
    xs.iter().filter(|&&x| x > 0).count()
}

pub fn summary(xs: &Vec<i32>) -> (i32, usize) {
    (total(xs), count_positive(xs))
}
