//! 06 (1x) - Ассоциированные константа и функция.
//!
//! Трейт `Bounded`: константы MIN_VALUE/MAX_VALUE и ассоциированная функция `midpoint()`
//! (без self! вызывается как `T::midpoint()` - диспетчеризация по типу результата).
//! Реализуйте для i8 и u8. В `snap_to_bound` обращайтесь к константам через параметр
//! типа: `T::MIN_VALUE`.
//!
//! Константам заглушка `todo!()` не подходит (const вычисляется на компиляции) - вместо
//! неё стоят значения-заполнители: замените их на правильные.

pub trait Bounded: Sized + PartialOrd {
    const MIN_VALUE: Self;
    const MAX_VALUE: Self;

    /// Середина диапазона типа.
    fn midpoint() -> Self;
}

impl Bounded for i8 {
    const MIN_VALUE: i8 = 0; // заполнитель: замените на настоящий минимум i8
    const MAX_VALUE: i8 = 0; // заполнитель: замените на настоящий максимум i8

    fn midpoint() -> i8 {
        todo!("(-128 + 127) / 2 == -1")
    }
}

impl Bounded for u8 {
    const MIN_VALUE: u8 = 1; // заполнитель
    const MAX_VALUE: u8 = 1; // заполнитель

    fn midpoint() -> u8 {
        todo!("127")
    }
}

/// Возвращает MIN_VALUE, если x < midpoint, иначе MAX_VALUE
/// ("прижать к ближайшей границе типа").
pub fn snap_to_bound<T: Bounded>(x: T) -> T {
    todo!("сравните с T::midpoint(), верните T::MIN_VALUE или T::MAX_VALUE")
}
