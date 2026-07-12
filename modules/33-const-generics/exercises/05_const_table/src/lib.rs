//! 05 (2x) - таблицы, вычисленные при компиляции.
//!
//! Предвычисли таблицы через `const fn`. Требуемое API:
//! - `build_pow2<const N: usize>() -> [u64; N]` - степени двойки (`table[i] == 2^i`);
//! - `popcount(x: u32) -> u32` - число единичных бит;
//! - `build_popcount_table() -> [u8; 256]` - popcount для всех байтов;
//! - константы `POW2: [u64; 8]` и `POPCOUNT: [u8; 256]`, заполненные этими функциями.
//!
//! Подсказки:
//! - в `const fn` циклы пиши через `while`;
//! - тело `todo!()` в `const fn` недопустимо, поэтому заглушки возвращают заведомо
//!   неверные значения - тесты падают на утверждениях, пока не решено.
//!
//! Про константные вычисления:
//! <https://doc.rust-lang.org/reference/const_eval.html>.
pub const fn build_pow2<const N: usize>() -> [u64; N] {
    [0u64; N]
}

pub const fn popcount(x: u32) -> u32 {
    let _ = x;
    0
}

pub const fn build_popcount_table() -> [u8; 256] {
    [0u8; 256]
}

pub const POW2: [u64; 8] = build_pow2::<8>();
pub const POPCOUNT: [u8; 256] = build_popcount_table();
