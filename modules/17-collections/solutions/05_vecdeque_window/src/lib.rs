//! 05 (1x) - VecDeque: скользящее окно. Эталонное решение.

use std::collections::VecDeque;

pub struct SlidingAverage {
    window: VecDeque<f64>,
    capacity: usize,
}

impl SlidingAverage {
    /// capacity > 0.
    pub fn new(capacity: usize) -> Self {
        SlidingAverage { window: VecDeque::with_capacity(capacity), capacity }
    }

    /// Добавляет значение, вытесняя старейшее при переполнении; возвращает текущее среднее.
    pub fn push(&mut self, value: f64) -> f64 {
        if self.window.len() == self.capacity {
            self.window.pop_front();
        }
        self.window.push_back(value);
        let sum: f64 = self.window.iter().sum();
        sum / self.window.len() as f64
    }

    pub fn len(&self) -> usize {
        self.window.len()
    }

    pub fn is_empty(&self) -> bool {
        self.window.is_empty()
    }
}
