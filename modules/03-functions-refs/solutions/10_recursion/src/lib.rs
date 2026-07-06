//! 10 (2x) - Рекурсивные функции. Эталонное решение.

pub fn fib(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

pub fn pow(base: u64, exp: u32) -> u64 {
    if exp == 0 {
        1
    } else {
        base * pow(base, exp - 1)
    }
}

pub fn sum_digits(n: u32) -> u32 {
    if n < 10 {
        n
    } else {
        n % 10 + sum_digits(n / 10)
    }
}
