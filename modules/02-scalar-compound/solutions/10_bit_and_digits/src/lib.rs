//! 10 (2x) — Биты и цифры. Эталонное решение.

pub fn count_set_bits(n: u32) -> u32 {
    n.count_ones()
}

pub fn digit_sum(n: u32) -> u32 {
    let mut m = n;
    let mut acc = 0;
    while m > 0 {
        acc += m % 10;
        m /= 10;
    }
    acc
}

pub fn is_power_of_two(n: u32) -> bool {
    n != 0 && n & (n - 1) == 0
}
