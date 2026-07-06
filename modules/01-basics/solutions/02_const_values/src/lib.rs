//! 02 (0x) - Константы `const`. Эталонное решение.

pub fn seconds_in_days(days: i64) -> i64 {
    const SECONDS_PER_DAY: i64 = 86_400;
    days * SECONDS_PER_DAY
}

pub fn minutes_to_seconds(minutes: i64) -> i64 {
    const SECONDS_PER_MINUTE: i64 = 60;
    minutes * SECONDS_PER_MINUTE
}

pub fn circle_area(r: f64) -> f64 {
    const PI: f64 = std::f64::consts::PI;
    PI * r * r
}
