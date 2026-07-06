//! 07 (1x) - Методы, потребляющие `self`.
//!
//! Метод с параметром `self` (не `&self`) ЗАБИРАЕТ владение объектом: после вызова исходная
//! переменная недоступна. Применяют для преобразований "в другой тип" и извлечения полей.

pub struct Temperature {
    pub celsius: f64,
}

impl Temperature {
    pub fn new(celsius: f64) -> Self {
        Temperature { celsius }
    }

    /// Потребляет температуру и возвращает её в Фаренгейтах: c * 9/5 + 32.
    pub fn into_fahrenheit(self) -> f64 {
        todo!()
    }

    /// Потребляет температуру и возвращает её в Кельвинах: c + 273.15.
    pub fn into_kelvin(self) -> f64 {
        todo!()
    }
}

pub struct Wrapper {
    inner: String,
}

impl Wrapper {
    pub fn new(inner: String) -> Self {
        Wrapper { inner }
    }

    /// Потребляет обёртку и ОТДАЁТ владение внутренней строкой (переместите поле из self).
    pub fn into_inner(self) -> String {
        todo!()
    }
}
