//! 04 (0x) - ПОЧИНИ СЛОМАННОЕ: ошибка E0425.
//!
//! Крейт НЕ КОМПИЛИРУЕТСЯ. `error[E0425]: cannot find value ... in this scope` почти всегда
//! означает опечатку в имени переменной. Компилятор даже подскажет похожее имя ("help: a local
//! variable with a similar name exists"). Найдите опечатку и исправьте.

/// Утраивает число.
pub fn triple(n: i32) -> i32 {
    let result = n * 3;
    reslt
}
