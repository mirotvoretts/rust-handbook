//! 02 (0x) - Константы `const`.
//!
//! `const` - именованное значение, известное на компиляции и встраиваемое по месту.
//! Объявите нужные константы (по конвенции `SCREAMING_SNAKE_CASE`) и используйте их.

/// Сколько секунд в заданном числе суток. Используйте `const` для числа секунд в сутках.
pub fn seconds_in_days(days: i64) -> i64 {
    todo!("const SECONDS_PER_DAY: i64 = 86_400; вернуть days * SECONDS_PER_DAY")
}

/// Переводит минуты в секунды через константу.
pub fn minutes_to_seconds(minutes: i64) -> i64 {
    todo!()
}

/// Площадь круга радиуса `r`. Используйте `const` для числа π
/// (можно `std::f64::consts::PI`).
pub fn circle_area(r: f64) -> f64 {
    todo!("PI * r * r")
}
