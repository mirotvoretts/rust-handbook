//! 05 (1x) — Методы `&mut self`, меняющие состояние. Эталонное решение.

pub struct Counter {
    count: u32,
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter {
    pub fn new() -> Self {
        Counter { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn add(&mut self, n: u32) {
        self.count += n;
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }

    pub fn value(&self) -> u32 {
        self.count
    }
}
