//! 05 (2x) - построение массива через `MaybeUninit`.
//!
//! Реализуй `labels()`, возвращающую `[String; 5]`, где элемент `i` равен строке
//! `format!("item-{i}")` (то есть "item-0", ..., "item-4"). Тип `String` не `Copy`
//! и не имеет дешёвого способа заполнить массив повторением, поэтому массив нужно
//! собрать поэлементно в неинициализированном буфере `[MaybeUninit<String>; 5]`, а
//! затем превратить его в `[String; 5]` (раздел 6 README).
//!
//! Ограничение: не используй `std::array::from_fn`, `Default` и повторение `[x; N]`
//! для String - цель упражнения именно в ручной инициализации через `MaybeUninit`.
//!
//! Конструкции за пределами теории:
//! - неинициализированная ячейка и её заполнение: MaybeUninit (uninit, write,
//!   assume_init) https://doc.rust-lang.org/std/mem/union.MaybeUninit.html
//! - превращение массива `[MaybeUninit<T>; N]` в `[T; N]` после заполнения можно
//!   сделать через std::mem::transmute
//!   https://doc.rust-lang.org/std/mem/fn.transmute.html

/// Построй `["item-0", ..., "item-4"]` поэлементно через `MaybeUninit`.
pub fn labels() -> [String; 5] {
    todo!()
}
