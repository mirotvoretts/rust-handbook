//! 03 (0x) — ПОЧИНИ СЛОМАННОЕ: ошибка E0384. Эталонное решение.
//!
//! Добавили `mut`: теперь переменной можно присваивать повторно.

pub fn countdown_from(start: i32) -> i32 {
    let mut counter = start;
    counter = counter - 1;
    counter = counter - 1;
    counter
}
