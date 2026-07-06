//! 03 (0x) - Блок как выражение. Эталонное решение.

pub fn hypot_sq(a: i32, b: i32) -> i32 {
    let a2 = a * a;
    let b2 = b * b;
    a2 + b2
}

pub fn celsius_to_fahrenheit(c: i32) -> i32 {
    c * 9 / 5 + 32
}

pub fn average3(a: i32, b: i32, c: i32) -> i32 {
    (a + b + c) / 3
}
