//! 03 (0x) - Числа с плавающей точкой `f64`. Эталонное решение.

pub fn average(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

pub fn is_close(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}

pub fn hypotenuse(a: f64, b: f64) -> f64 {
    (a * a + b * b).sqrt()
}
