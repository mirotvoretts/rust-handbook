//! 01 (0x) - Arc<Mutex<T>>: общий счётчик. Эталонное решение.

use std::sync::{Arc, Mutex};
use std::thread;

pub fn concurrent_increment(n_threads: usize, per_thread: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::with_capacity(n_threads);

    for _ in 0..n_threads {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..per_thread {
                let mut guard = c.lock().unwrap();
                *guard += 1;
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let total = *counter.lock().unwrap();
    total
}
