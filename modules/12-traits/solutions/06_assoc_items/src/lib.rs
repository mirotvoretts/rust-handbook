//! 06 (1x) — Ассоциированные константа и функция. Эталонное решение.

pub trait Bounded: Sized + PartialOrd {
    const MIN_VALUE: Self;
    const MAX_VALUE: Self;

    /// Середина диапазона. Без self: диспетчеризация по типу результата.
    fn midpoint() -> Self;
}

impl Bounded for i8 {
    const MIN_VALUE: i8 = i8::MIN;
    const MAX_VALUE: i8 = i8::MAX;

    fn midpoint() -> i8 {
        -1
    }
}

impl Bounded for u8 {
    const MIN_VALUE: u8 = u8::MIN;
    const MAX_VALUE: u8 = u8::MAX;

    fn midpoint() -> u8 {
        127
    }
}

/// Возвращает MIN_VALUE, если x < midpoint, иначе MAX_VALUE.
pub fn snap_to_bound<T: Bounded>(x: T) -> T {
    if x < T::midpoint() {
        T::MIN_VALUE
    } else {
        T::MAX_VALUE
    }
}
