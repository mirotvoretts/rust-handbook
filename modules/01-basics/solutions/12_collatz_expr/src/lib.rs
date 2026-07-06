//! 12 (3x) — Коллатц через выражения, без `return`. Эталонное решение.

pub fn collatz_max(start: u64) -> u64 {
    let mut n = start;
    let mut max = start;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        if n > max {
            max = n;
        }
    }
    max
}

pub fn collatz_len(start: u64) -> u64 {
    let mut n = start;
    let mut len = 1;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}
