//! 04 (1x) — Методы обобщённой структуры. Эталонное решение.

/// Обёртка над одним значением.
pub struct Holder<T> {
    value: T,
}

impl<T> Holder<T> {
    /// Упаковывает значение.
    pub fn new(value: T) -> Self {
        Holder { value }
    }

    /// Ссылка на содержимое.
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Забирает содержимое, поглощая обёртку.
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Преобразует содержимое функцией, меняя тип: Holder<T> -> Holder<U>.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Holder<U> {
        Holder { value: f(self.value) }
    }
}
