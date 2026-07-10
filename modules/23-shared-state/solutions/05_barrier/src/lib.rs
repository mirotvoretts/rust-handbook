//! 05 (2x) - барьер на Mutex + Condvar. Эталонное решение.

use std::sync::{Condvar, Mutex};

pub struct Barrier {
    n: usize,
    arrived: Mutex<usize>,
    cv: Condvar,
}

impl Barrier {
    pub fn new(n: usize) -> Self {
        Barrier {
            n,
            arrived: Mutex::new(0),
            cv: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut count = self.arrived.lock().unwrap();
        *count += 1;
        if *count == self.n {
            // последний пришедший будит всех остальных
            self.cv.notify_all();
        } else {
            let target = self.n;
            // цикл while защищает от ложных пробуждений
            while *count < target {
                count = self.cv.wait(count).unwrap();
            }
        }
    }
}
