//! 06 (2x) - scope над чанками, максимум по частям. Эталонное решение.

use std::thread;

/// Параллельный максимум по не более чем parts потокам.
pub fn parallel_max(xs: &[i32], parts: usize) -> Option<i32> {
    if xs.is_empty() {
        return None;
    }
    let parts = parts.max(1);
    let chunk = xs.len().div_ceil(parts);
    thread::scope(|s| {
        let handles: Vec<_> = xs
            .chunks(chunk)
            .map(|c| s.spawn(move || c.iter().copied().max().unwrap()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).max()
    })
}
