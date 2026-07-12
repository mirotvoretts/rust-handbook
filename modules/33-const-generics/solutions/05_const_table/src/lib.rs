//! 05 (2x) - таблицы, вычисленные при компиляции. Эталонное решение.
pub const fn build_pow2<const N: usize>() -> [u64; N] {
    let mut table = [0u64; N];
    let mut i = 0;
    while i < N {
        table[i] = 1u64 << i;
        i += 1;
    }
    table
}

pub const fn popcount(mut x: u32) -> u32 {
    let mut count = 0;
    while x != 0 {
        count += x & 1;
        x >>= 1;
    }
    count
}

pub const fn build_popcount_table() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        table[i] = popcount(i as u32) as u8;
        i += 1;
    }
    table
}

pub const POW2: [u64; 8] = build_pow2::<8>();
pub const POPCOUNT: [u8; 256] = build_popcount_table();
