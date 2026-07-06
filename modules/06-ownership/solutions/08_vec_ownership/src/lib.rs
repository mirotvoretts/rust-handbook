//! 08 (1x) - Владение и `Vec`. Эталонное решение.

pub fn sum_owned(v: Vec<i32>) -> i32 {
    v.into_iter().sum()
}

pub fn sum_borrowed(v: &[i32]) -> i32 {
    v.iter().sum()
}

pub fn doubled(v: &[i32]) -> Vec<i32> {
    v.iter().map(|x| x * 2).collect()
}

pub fn join_owned(v: Vec<String>, sep: &str) -> String {
    v.join(sep)
}
