//! 09 (2x) - Проход по срезу с соседними элементами. Эталонное решение.

pub fn is_sorted_asc(xs: &[i32]) -> bool {
    for i in 1..xs.len() {
        if xs[i - 1] > xs[i] {
            return false;
        }
    }
    true
}

pub fn count_adjacent_equal(xs: &[i32]) -> usize {
    let mut count = 0;
    for i in 1..xs.len() {
        if xs[i - 1] == xs[i] {
            count += 1;
        }
    }
    count
}

pub fn pairwise_sums(xs: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    for i in 1..xs.len() {
        out.push(xs[i - 1] + xs[i]);
    }
    out
}
