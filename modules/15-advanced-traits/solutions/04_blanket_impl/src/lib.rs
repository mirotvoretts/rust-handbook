//! 04 (2x) - Blanket impl. Эталонное решение.

use std::fmt;

pub trait Loggable {
    fn log_line(&self) -> String;
}

impl<T: fmt::Display> Loggable for T {
    fn log_line(&self) -> String {
        format!("[LOG] {self}")
    }
}

/// Свой тип: Display даёт ему Loggable автоматически.
pub struct Temperature(pub f64);

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}C", self.0)
    }
}
