//! 05 (1x) — `loop` с возвратом значения. Эталонное решение.

pub fn first_pow2_at_least(n: u32) -> u32 {
    let mut p = 1;
    loop {
        if p >= n {
            break p;
        }
        p *= 2;
    }
}

pub fn sum_1_to(n: u32) -> u32 {
    let mut i = 1;
    let mut acc = 0;
    loop {
        if i > n {
            break acc;
        }
        acc += i;
        i += 1;
    }
}
