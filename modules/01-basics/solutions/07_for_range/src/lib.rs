//! 07 (1x) — `for` по диапазону. Эталонное решение.

pub fn factorial(n: u64) -> u64 {
    let mut acc = 1;
    for i in 1..=n {
        acc *= i;
    }
    acc
}

pub fn sum_multiples_below(limit: u32) -> u32 {
    let mut acc = 0;
    for i in 1..limit {
        if i % 3 == 0 || i % 5 == 0 {
            acc += i;
        }
    }
    acc
}
