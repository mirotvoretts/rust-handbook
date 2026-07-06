//! 05 (1x) - VecDeque: скользящее окно.
//!
//! Кольцевой буфер: push_back + pop_front за O(1) (у Vec pop спереди - O(n)).
//! SlidingAverage хранит последние capacity значений и отдаёт среднее.

use std::collections::VecDeque;

pub struct SlidingAverage {
    window: VecDeque<f64>,
    capacity: usize,
}

impl SlidingAverage {
    /// capacity > 0.
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    /// Добавляет значение, вытесняя старейшее при переполнении; возвращает текущее среднее.
    pub fn push(&mut self, value: f64) -> f64 {
        todo!("при len == capacity сначала pop_front; затем push_back и среднее по итератору")
    }

    pub fn len(&self) -> usize {
        todo!()
    }
}
