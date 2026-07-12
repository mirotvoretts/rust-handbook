//! 01 (0x) - вызов функции C из Rust через `extern "C"`. Эталонное решение.

use std::os::raw::c_int;

extern "C" {
    // int abs(int); из стандартной библиотеки C.
    fn abs(input: c_int) -> c_int;
}

/// Модуль числа через libc `abs`.
pub fn libc_abs(x: c_int) -> c_int {
    // SAFETY: abs тотальна для любого c_int, кроме c_int::MIN; тесты его не передают.
    unsafe { abs(x) }
}

/// Модуль разности `a - b`.
pub fn abs_diff(a: c_int, b: c_int) -> c_int {
    libc_abs(a.wrapping_sub(b))
}
