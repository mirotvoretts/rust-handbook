//! 04 (2x) - Blanket impl.
//!
//! Один impl - и Loggable получают ВСЕ типы с Display: и i32, и ваш Temperature.
//! Формат: "[LOG] {значение}". Второй impl для конкретного типа писать нельзя -
//! конфликт (E0119); кастомизация - только через Display.

use std::fmt;

pub trait Loggable {
    fn log_line(&self) -> String;
}

// todo!(): blanket impl - impl<T: fmt::Display> Loggable for T
impl<T: fmt::Display> Loggable for T {
    fn log_line(&self) -> String {
        todo!()
    }
}

/// Свой тип: Display даёт ему Loggable автоматически.
pub struct Temperature(pub f64);

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("градусы с одним знаком: 21.5C")
    }
}
