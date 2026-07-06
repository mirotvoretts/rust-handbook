//! 08 (3x) - Свой бесконечный итератор + IntoIterator + пайплайн. Эталонное решение.

/// Бесконечный ряд Фибоначчи.
pub struct Fib {
    a: u64,
    b: u64,
}

impl Fib {
    /// Ряд с 0, 1.
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

    fn next(&mut self) -> Option<u64> {
        let cur = self.a;
        self.a = self.b;
        self.b = cur.saturating_add(self.b);
        Some(cur)
    }
}

/// Первые n чисел Фибоначчи.
pub fn first_fibs(n: usize) -> Vec<u64> {
    Fib::new().take(n).collect()
}

/// Сумма чётных чисел Фибоначчи меньше limit.
pub fn sum_even_fibs_below(limit: u64) -> u64 {
    Fib::new()
        .take_while(|&x| x < limit)
        .filter(|x| x % 2 == 0)
        .sum()
}

/// Мешок чисел, обходимый в for.
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

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
