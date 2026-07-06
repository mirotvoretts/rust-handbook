//! 09 (2x) - Преобразования `&mut [i32]` на месте с накоплением. Эталонное решение.

pub fn running_max_in_place(xs: &mut [i32]) {
    for i in 1..xs.len() {
        if xs[i - 1] > xs[i] {
            xs[i] = xs[i - 1];
        }
    }
}

pub fn prefix_sums_in_place(xs: &mut [i32]) {
    for i in 1..xs.len() {
        xs[i] += xs[i - 1];
    }
}
