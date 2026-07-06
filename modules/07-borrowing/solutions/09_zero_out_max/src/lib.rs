//! 09 (2x) - Найти индекс максимума, затем обнулить его. Эталонное решение.

pub fn zero_out_max(xs: &mut [i32]) {
    if xs.is_empty() {
        return;
    }
    let mut idx = 0;
    for i in 1..xs.len() {
        if xs[i] > xs[idx] {
            idx = i;
        }
    }
    xs[idx] = 0;
}
