//! 08 (3x) - Капстоун: свой БЕСКОНЕЧНЫЙ итератор + IntoIterator + пайплайн.
//!
//! Собирает всё вместе: (1) реализовать Iterator, который никогда не отдаёт None
//! (обрезает его потребитель через take/take_while); (2) реализовать IntoIterator
//! для своего типа, чтобы он работал в for; (3) построить конвейер-потребитель.

/// Бесконечный итератор Фибоначчи: 0, 1, 1, 2, 3, 5, 8, 13, ...
/// next() никогда не вернёт None. Значения u64; при переполнении - насыщение
/// (saturating_add), чтобы бесконечный потребитель не паниковал.
pub struct Fib {
    a: u64,
    b: u64,
}

impl Fib {
    /// Новый ряд, стартующий с 0, 1.
    pub fn new() -> Self {
        Fib { a: 0, b: 1 }
    }
}

impl Default for Fib {
    fn default() -> Self {
        Fib::new()
    }
}

impl Iterator for Fib {
    type Item = u64;

    /// Верни текущее a, затем сдвинь пару: (a, b) -> (b, a+b). Всегда Some.
    fn next(&mut self) -> Option<u64> {
        todo!("let cur = self.a; self.a = self.b; self.b = cur.saturating_add(self.b); Some(cur)")
    }
}

/// Первые n чисел Фибоначчи. Обрежь бесконечный итератор через take.
pub fn first_fibs(n: usize) -> Vec<u64> {
    todo!("Fib::new().take(n).collect()")
}

/// Сумма ЧЁТНЫХ чисел Фибоначчи, строго меньших limit.
/// Останови обход, как только значения достигнут limit (take_while), из
/// оставшихся возьми чётные и просуммируй.
pub fn sum_even_fibs_below(limit: u64) -> u64 {
    todo!("Fib::new().take_while(|&x| x < limit).filter(|x| x % 2 == 0).sum()")
}

/// Мешок чисел - обёртка над Vec, которую можно обходить в for напрямую.
pub struct Bag {
    items: Vec<i32>,
}

impl Bag {
    /// Собрать мешок.
    pub fn new(items: Vec<i32>) -> Self {
        Bag { items }
    }
}

impl IntoIterator for Bag {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    /// Отдай владение внутренним вектором, делегируя его into_iter.
    fn into_iter(self) -> Self::IntoIter {
        todo!("self.items.into_iter()")
    }
}
