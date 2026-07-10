//! 08 (3x) - counting semaphore на Mutex + Condvar. Эталонное решение.

use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    permits: Mutex<usize>,
    cv: Condvar,
}

impl Semaphore {
    pub fn new(permits: usize) -> Self {
        Semaphore {
            permits: Mutex::new(permits),
            cv: Condvar::new(),
        }
    }

    pub fn acquire(&self) {
        let mut p = self.permits.lock().unwrap();
        while *p == 0 {
            p = self.cv.wait(p).unwrap(); // спим, пока разрешений нет
        }
        *p -= 1;
    }

    pub fn release(&self) {
        let mut p = self.permits.lock().unwrap();
        *p += 1;
        self.cv.notify_one();
    }
}
