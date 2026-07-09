//! 01 (0x) - Cell<T>: замена значения целиком. Эталонное решение.

use std::cell::Cell;

/// Увеличь значение внутри Cell на 1.
pub fn bump(c: &Cell<i32>) {
    c.set(c.get() + 1);
}

/// Замени значение на new, верни прежнее.
pub fn replace_get(c: &Cell<i32>, new: i32) -> i32 {
    c.replace(new)
}

/// Забери значение, оставив Default (0).
pub fn take(c: &Cell<i32>) -> i32 {
    c.take()
}
