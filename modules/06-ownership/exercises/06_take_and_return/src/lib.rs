//! 06 (1x) - Consuming builder: методы, забирающие `self` по значению.
//!
//! Паттерн "взять self, вернуть изменённый self" позволяет строить цепочки
//! `Config::new().with_name("x").with_verbose(true)`. Каждый метод потребляет старый объект
//! (move) и возвращает новый - промежуточные значения не остаются доступными.
//!
//! Реализуйте `new` и оба метода `with_*`. Обратите внимание на `self` (без `&`) в сигнатуре.

#[derive(Debug, PartialEq)]
pub struct Config {
    pub name: String,
    pub verbose: bool,
    pub retries: u32,
}

impl Config {
    /// Значения по умолчанию: пустое имя, verbose = false, retries = 0.
    pub fn new() -> Config {
        todo!()
    }

    /// Возвращает конфиг с установленным именем (потребляя `self`).
    pub fn with_name(self, name: &str) -> Config {
        todo!()
    }

    /// Возвращает конфиг с установленным флагом verbose (потребляя `self`).
    pub fn with_verbose(self, verbose: bool) -> Config {
        todo!()
    }

    /// Возвращает конфиг с установленным числом повторов (потребляя `self`).
    pub fn with_retries(self, retries: u32) -> Config {
        todo!()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}
