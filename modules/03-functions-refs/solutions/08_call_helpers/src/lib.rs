//! 08 (1x) — Вызов одной функции из другой. Эталонное решение.

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

pub fn count_primes_below(limit: u32) -> usize {
    let mut count = 0;
    for n in 2..limit {
        if is_prime(n) {
            count += 1;
        }
    }
    count
}
