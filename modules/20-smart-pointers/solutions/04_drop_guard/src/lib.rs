//! 04 (1x) - Drop: RAII-гвард. Эталонное решение.

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// RAII-гвард поверх общего счётчика.
pub struct Guard {
    counter: Rc<AtomicUsize>,
}

impl Guard {
    /// Создать гвард.
    pub fn new(counter: Rc<AtomicUsize>) -> Self {
        Guard { counter }
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }
}

/// Текущее значение счётчика.
pub fn dropped(counter: &Rc<AtomicUsize>) -> usize {
    counter.load(Ordering::SeqCst)
}
