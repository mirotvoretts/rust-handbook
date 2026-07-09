//! 04 (1x) - много потоков, результаты по порядку. Эталонное решение.

use std::thread;

/// Квадрат каждого элемента в своём потоке, порядок сохраняется.
pub fn squares(xs: Vec<i64>) -> Vec<i64> {
    let handles: Vec<_> = xs
        .into_iter()
        .map(|x| thread::spawn(move || x * x))
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}
