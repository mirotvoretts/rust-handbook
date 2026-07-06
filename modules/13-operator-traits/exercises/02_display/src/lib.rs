//! 02 (0x) — Display для температуры.
//!
//! Display — представление «для людей», derive для него не бывает. Формат:
//! ровно один знак после запятой + "°C", например "36.6°C" и "-5.0°C".
//! Подсказка: спецификатор точности в format-строке — двоеточие-точка-один.
//! Бонус за понимание: откуда у Celsius возьмётся метод .to_string()?

use std::fmt;

pub struct Celsius(pub f64);

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("write! в f")
    }
}
