//! 06 (1x) — Цикл `while`. Эталонное решение.

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

pub fn collatz_steps(mut n: u64) -> u64 {
    let mut steps = 0;
    while n != 1 {
        n = if n.is_multiple_of(2) { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }
    steps
}
