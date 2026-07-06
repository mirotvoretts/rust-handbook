//! 09 (2x) - Минимальное ядро + default-методы.
//!
//! Спроектируйте трейт как Iterator: ОДНО обязательное `values()` и default-методы
//! поверх него. Реализуйте default'ы в самом трейте (count/total/mean), а затем два
//! impl. `Fibs` показывает силу подхода: тип, ВЫЧИСЛЯЮЩИЙ значения, получает всю
//! статистику бесплатно.

pub trait Stats {
    /// Ядро трейта: набор значений.
    fn values(&self) -> Vec<f64>;

    /// Сколько значений.
    fn count(&self) -> usize {
        todo!("через self.values()")
    }

    /// Сумма (0.0 для пустого набора).
    fn total(&self) -> f64 {
        todo!()
    }

    /// Среднее (None для пустого набора).
    fn mean(&self) -> Option<f64> {
        todo!("total / count, но осторожно с пустым")
    }
}

/// Простое хранилище готовых значений.
pub struct Samples(pub Vec<f64>);

impl Stats for Samples {
    fn values(&self) -> Vec<f64> {
        todo!("клонировать внутренний вектор")
    }
}

/// Первые n чисел Фибоначчи (1, 1, 2, 3, 5, ...) как f64.
pub struct Fibs(pub usize);

impl Stats for Fibs {
    fn values(&self) -> Vec<f64> {
        todo!("сгенерируйте первые self.0 чисел")
    }
}
