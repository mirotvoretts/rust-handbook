//! 03 (0x) - ПОЧИНИ СЛОМАННОЕ: ошибка E0384.
//!
//! Крейт НЕ КОМПИЛИРУЕТСЯ. Ошибка `error[E0384]: cannot assign twice to immutable variable`
//! означает: переменная неизменяема по умолчанию, а вы ей повторно присваиваете. Разрешите
//! изменение, добавив `mut` в объявление переменной.

/// Возвращает `start`, дважды уменьшенное на 1 (то есть start - 2).
pub fn countdown_from(start: i32) -> i32 {
    let counter = start;
    counter = counter - 1;
    counter = counter - 1;
    counter
}
