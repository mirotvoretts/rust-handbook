//! 01 (0x) - Объявление функций, возврат, ранний `return`. Эталонное решение.

pub fn square(n: i32) -> i32 {
    n * n
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn abs_val(n: i32) -> i32 {
    if n < 0 {
        return -n;
    }
    n
}
