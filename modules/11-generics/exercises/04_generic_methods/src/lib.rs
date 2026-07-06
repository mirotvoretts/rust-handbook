//! 04 (1x) — Методы обобщённой структуры.
//!
//! `impl<T> Holder<T>`: первое `<T>` объявляет параметр, второе использует. Метод `map`
//! вводит СОБСТВЕННЫЙ параметр `U` поверх параметра структуры и меняет тип содержимого:
//! `Holder<T>` -> `Holder<U>`.

/// Обёртка над одним значением.
pub struct Holder<T> {
    value: T,
}

impl<T> Holder<T> {
    /// Упаковывает значение.
    pub fn new(value: T) -> Self {
        todo!()
    }

    /// Ссылка на содержимое.
    pub fn get(&self) -> &T {
        todo!()
    }

    /// Забирает содержимое, поглощая обёртку.
    pub fn into_inner(self) -> T {
        todo!()
    }

    /// Преобразует содержимое функцией, меняя тип: Holder<T> -> Holder<U>.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Holder<U> {
        todo!("f(self.value) и завернуть обратно")
    }
}
