//! 08 (1x) — Прочитать минимум, затем изменить все элементы. Эталонное решение.

pub fn shift_by_min(xs: &mut [i32]) {
    let min = match xs.iter().min() {
        Some(&m) => m,
        None => return, // пустой срез
    };
    for x in xs.iter_mut() {
        *x -= min;
    }
}
