//! 05 (1x) - Возврат `Option<&T>`. Эталонное решение.

pub fn get(xs: &[i32], i: usize) -> Option<&i32> {
    xs.get(i)
}

pub fn find(xs: &[i32], target: i32) -> Option<&i32> {
    xs.iter().find(|&&x| x == target)
}
