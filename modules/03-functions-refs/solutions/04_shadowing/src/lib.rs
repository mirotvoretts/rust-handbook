//! 04 (0x) - Shadowing (затенение). Эталонное решение.

#[allow(clippy::let_and_return)] // намеренно: демонстрирует полную цепочку из 3 затенений `n`
pub fn shadow_math(n: i32) -> i32 {
    let n = n + 5;
    let n = n * 2;
    let n = n - 1;
    n
}

pub fn trimmed_len(s: &str) -> usize {
    let s = s.trim();
    s.chars().count()
}
