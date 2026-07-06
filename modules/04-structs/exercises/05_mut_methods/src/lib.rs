//! 05 (1x) — Методы `&mut self`, меняющие состояние.
//!
//! Методы, изменяющие поля, берут `&mut self`. Вызвавшая переменная должна быть `let mut`.

pub struct Counter {
    count: u32,
}

impl Counter {
    /// Новый счётчик со значением 0.
    pub fn new() -> Self {
        todo!()
    }

    /// Увеличивает счётчик на 1.
    pub fn increment(&mut self) {
        todo!()
    }

    /// Прибавляет к счётчику `n`.
    pub fn add(&mut self, n: u32) {
        todo!()
    }

    /// Сбрасывает счётчик в 0.
    pub fn reset(&mut self) {
        todo!()
    }

    /// Текущее значение (только читает).
    pub fn value(&self) -> u32 {
        todo!()
    }
}
