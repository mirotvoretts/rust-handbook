//! 08 (3x) - непересекающиеся &mut через scope. Эталонное решение.

use std::thread;

/// Удвоить срез на месте, раздав непересекающиеся изменяемые куски потокам.
pub fn parallel_double(xs: &mut [i32], parts: usize) {
    if xs.is_empty() {
        return;
    }
    let chunk = xs.len().div_ceil(parts.max(1));
    thread::scope(|s| {
        for c in xs.chunks_mut(chunk) {
            s.spawn(move || {
                for x in c.iter_mut() {
                    *x *= 2;
                }
            });
        }
    });
}
