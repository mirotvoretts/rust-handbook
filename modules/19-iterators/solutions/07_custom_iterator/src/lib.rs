//! 07 (2x) - Свой итератор. Эталонное решение.

/// Обратный отсчёт n, n-1, ..., 1.
pub struct Countdown {
    n: u32,
}

impl Countdown {
    /// Стартовать с n.
    pub fn from(n: u32) -> Self {
        Countdown { n }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.n == 0 {
            None
        } else {
            let cur = self.n;
            self.n -= 1;
            Some(cur)
        }
    }
}

/// Сумма квадратов обратного отсчёта - через адаптеры своего итератора.
pub fn sum_of_squares_countdown(n: u32) -> u64 {
    Countdown::from(n).map(|x| (x as u64).pow(2)).sum()
}
