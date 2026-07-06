//! 01 (0x) - Привязки `let`/`mut`. Эталонное решение.

pub fn increment_thrice(start: i32) -> i32 {
    let mut acc = start;
    acc += 1;
    acc += 1;
    acc += 1;
    acc
}

pub fn running_total(a: i32, b: i32, c: i32) -> i32 {
    let mut t = 0;
    t += a;
    t += b;
    t += c;
    t
}

pub fn swapped(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}
