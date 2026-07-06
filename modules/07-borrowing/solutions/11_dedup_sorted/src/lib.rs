//! 11 (2x) — Убрать соседние дубликаты на месте. Эталонное решение.

pub fn dedup_sorted(xs: &mut Vec<i32>) {
    let mut result: Vec<i32> = Vec::new();
    for &x in xs.iter() {
        if result.last() != Some(&x) {
            result.push(x);
        }
    }
    *xs = result;
}
