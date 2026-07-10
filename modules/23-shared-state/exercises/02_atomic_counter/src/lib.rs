//! 02 (0x) - атомарный счётчик без замка.
//!
//! AtomicUsize::fetch_add атомарно прибавляет значение и возвращает прежнее - весь
//! read-modify-write неделим, поэтому Mutex не нужен. Для простого счёта, где важен
//! лишь итог, достаточно Ordering::Relaxed. Атомик делят между потоками через Arc.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// То же, что в упражнении 01, но общий счётчик - AtomicUsize вместо Mutex.
/// Запусти `n_threads` потоков, каждый `per_thread` раз атомарно прибавляет 1,
/// дождись всех и верни итог.
pub fn atomic_increment(n_threads: usize, per_thread: usize) -> usize {
    todo!()
}
