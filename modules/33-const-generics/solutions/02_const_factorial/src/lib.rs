//! 02 (0x) - const fn factorial и fib. Эталонное решение.
pub const fn factorial(n: u64) -> u64 {
    let mut result = 1;
    let mut i = 2;
    while i <= n {
        result *= i;
        i += 1;
    }
    result
}

pub const fn fib(n: u64) -> u64 {
    let mut a = 0u64;
    let mut b = 1u64;
    let mut i = 0;
    while i < n {
        let next = a + b;
        a = b;
        b = next;
        i += 1;
    }
    a
}
