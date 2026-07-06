//! 06 (1x) — Паника и тест `#[should_panic]`. Эталонное решение.

pub fn safe_div_100(n: i32) -> i32 {
    if n == 0 {
        panic!("division by zero");
    }
    100 / n
}
