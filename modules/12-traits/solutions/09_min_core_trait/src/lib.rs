//! 09 (2x) - Минимальное ядро + default-методы. Эталонное решение.

pub trait Stats {
    /// Ядро трейта: набор значений.
    fn values(&self) -> Vec<f64>;

    /// Сколько значений.
    fn count(&self) -> usize {
        self.values().len()
    }

    /// Сумма (0.0 для пустого набора).
    fn total(&self) -> f64 {
        let mut sum = 0.0;
        for v in self.values() {
            sum += v;
        }
        sum
    }

    /// Среднее (None для пустого набора).
    fn mean(&self) -> Option<f64> {
        let n = self.count();
        if n == 0 {
            None
        } else {
            Some(self.total() / n as f64)
        }
    }
}

/// Простое хранилище готовых значений.
pub struct Samples(pub Vec<f64>);

impl Stats for Samples {
    fn values(&self) -> Vec<f64> {
        self.0.clone()
    }
}

/// Первые n чисел Фибоначчи (1, 1, 2, 3, 5, ...) как f64.
pub struct Fibs(pub usize);

impl Stats for Fibs {
    fn values(&self) -> Vec<f64> {
        let mut out = Vec::new();
        let (mut a, mut b) = (1u64, 1u64);
        for _ in 0..self.0 {
            out.push(a as f64);
            let next = a + b;
            a = b;
            b = next;
        }
        out
    }
}
