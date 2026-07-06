//! 02 (0x) - Изменение через единственный `&mut`. Эталонное решение.

pub fn push_doubled(xs: &mut Vec<i32>, value: i32) {
    xs.push(value * 2);
}

pub fn scale_all(xs: &mut [i32], k: i32) {
    for x in xs.iter_mut() {
        *x *= k;
    }
}
