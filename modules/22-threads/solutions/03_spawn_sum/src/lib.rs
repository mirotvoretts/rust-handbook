//! 03 (1x) - переместить вектор в поток, вернуть сумму. Эталонное решение.

use std::thread;

/// Сумма элементов в отдельном потоке, владеющем вектором.
pub fn spawn_sum(xs: Vec<i32>) -> i64 {
    let handle = thread::spawn(move || xs.iter().map(|&x| x as i64).sum::<i64>());
    handle.join().unwrap()
}
