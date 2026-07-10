//! 06 (2x) - compare_exchange: счётчик с потолком. Эталонное решение.

use std::sync::atomic::{AtomicUsize, Ordering};

pub struct SeatCounter {
    taken: AtomicUsize,
    capacity: usize,
}

impl SeatCounter {
    pub fn new(capacity: usize) -> Self {
        SeatCounter {
            taken: AtomicUsize::new(0),
            capacity,
        }
    }

    pub fn try_take(&self) -> bool {
        let mut cur = self.taken.load(Ordering::Relaxed);
        loop {
            if cur >= self.capacity {
                return false;
            }
            match self.taken.compare_exchange_weak(
                cur,
                cur + 1,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => return true,
                Err(actual) => cur = actual, // промах: кто-то опередил, повторяем
            }
        }
    }

    pub fn taken(&self) -> usize {
        self.taken.load(Ordering::Acquire)
    }
}
