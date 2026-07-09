//! 01 (0x) - spawn + join. Эталонное решение.

use std::thread;

/// Вычислить a + b в отдельном потоке.
pub fn spawn_add(a: i32, b: i32) -> i32 {
    let handle = thread::spawn(move || a + b);
    handle.join().unwrap()
}
