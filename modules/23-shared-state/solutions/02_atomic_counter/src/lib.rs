//! 02 (0x) - атомарный счётчик. Эталонное решение.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub fn atomic_increment(n_threads: usize, per_thread: usize) -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(n_threads);

    for _ in 0..n_threads {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..per_thread {
                c.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    counter.load(Ordering::Relaxed)
}
