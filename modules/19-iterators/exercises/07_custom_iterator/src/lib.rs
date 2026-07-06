//! 07 (2x) - Свой итератор: impl Iterator для своего типа.
//!
//! Реализуешь один метод next - и получаешь ВСЕ адаптеры/потребители даром
//! (у них тела по умолчанию в трейте). Итератор хранит состояние обхода в полях.

/// Обратный отсчёт: выдаёт n, n-1, ..., 1, затем None (0 не выдаётся).
pub struct Countdown {
    n: u32,
}

impl Countdown {
    /// Стартовать отсчёт с n.
    pub fn from(n: u32) -> Self {
        Countdown { n }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    /// Пока n > 0: уменьши n и верни новое значение? Нет - верни ТЕКУЩЕЕ n,
    /// затем уменьшай. Итог: n, n-1, ..., 1, None.
    fn next(&mut self) -> Option<u32> {
        todo!("if self.n == 0 { None } else { let cur = self.n; self.n -= 1; Some(cur) }")
    }
}

/// Докажи, что адаптеры бесплатны: сумма КВАДРАТОВ обратного отсчёта от n,
/// посчитанная через map + sum на своём итераторе. Тип суммы - u64.
pub fn sum_of_squares_countdown(n: u32) -> u64 {
    todo!("Countdown::from(n).map(|x| (x as u64).pow(2)).sum()")
}
