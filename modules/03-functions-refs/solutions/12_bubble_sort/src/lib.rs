//! 12 (3x) — Сортировка пузырьком на месте. Эталонное решение.

pub fn is_sorted_asc(xs: &[i32]) -> bool {
    for i in 1..xs.len() {
        if xs[i - 1] > xs[i] {
            return false;
        }
    }
    true
}

pub fn bubble_sort(xs: &mut [i32]) {
    let n = xs.len();
    loop {
        let mut swapped = false;
        for i in 1..n {
            if xs[i - 1] > xs[i] {
                xs.swap(i - 1, i);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
