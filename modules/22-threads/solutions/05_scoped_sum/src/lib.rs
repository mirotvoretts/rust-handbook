//! 05 (2x) - thread::scope, сумма двух половин. Эталонное решение.

use std::thread;

/// Сумма среза в двух scoped-потоках.
pub fn parallel_sum(xs: &[i64]) -> i64 {
    let mid = xs.len() / 2;
    let (left, right) = xs.split_at(mid);
    thread::scope(|s| {
        let hl = s.spawn(|| left.iter().sum::<i64>());
        let hr = s.spawn(|| right.iter().sum::<i64>());
        hl.join().unwrap() + hr.join().unwrap()
    })
}
