//! 04 (0x) — Сокращение пути через `use`. Эталонное решение.

mod util {
    pub fn double(n: i32) -> i32 {
        n * 2
    }

    pub fn triple(n: i32) -> i32 {
        n * 3
    }
}

use util::{double, triple};

pub fn combine(n: i32) -> i32 {
    double(n) + triple(n)
}
