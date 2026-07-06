//! 10 (2x) - Цикл с изменяемым состоянием. Эталонное решение.

pub fn int_sqrt(n: u64) -> u64 {
    let mut r = 0;
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

pub fn reverse_digits(n: u32) -> u32 {
    let mut m = n;
    let mut rev = 0;
    while m > 0 {
        rev = rev * 10 + m % 10;
        m /= 10;
    }
    rev
}
