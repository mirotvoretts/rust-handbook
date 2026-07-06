//! 06 (2x) — dyn Any и downcast.
//!
//! Мешок значений неизвестных типов: Vec<Box<dyn Any>>. Рассортируйте его:
//! downcast_ref::<T>() возвращает Option<&T>, downcast::<T>() на Box — Result с
//! владением. Соберите все i32 (суммой) и все String (склейкой), остальное посчитайте.

use std::any::Any;

pub struct Sorted {
    pub int_sum: i64,
    pub strings_joined: String,
    pub unknown_count: usize,
}

/// Разбирает мешок: i32 суммируются, String склеиваются (в порядке следования,
/// без разделителя), прочее считается.
pub fn sort_bag(bag: Vec<Box<dyn Any>>) -> Sorted {
    todo!("downcast_ref::<i32>() / downcast_ref::<String>() в цепочке if let ... else if")
}
