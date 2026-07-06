//! 02 (0x) — Display для температуры. Эталонное решение.

use std::fmt;

pub struct Celsius(pub f64);

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}
