//! 01 (0x) - Синтаксис замыканий. Эталонное решение.

/// Замыкание без захватов: возведи x в квадрат.
pub fn square(x: i32) -> i32 {
    let f = |n| n * n;
    f(x)
}

/// Захват по чтению: a и b в окружении замыкания.
pub fn sum_via_capture(a: i32, b: i32) -> i32 {
    let f = || a + b;
    f()
}

/// Захват &str по чтению: замыкание строит приветствие.
pub fn greet(name: &str) -> String {
    let f = || format!("Hello, {name}!");
    f()
}
