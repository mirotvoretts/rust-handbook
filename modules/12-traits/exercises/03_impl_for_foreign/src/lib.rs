//! 03 (0x) - Свой трейт для чужих типов.
//!
//! Трейт `Sized`? Нет - `Weight`: "вес" значения. Реализуйте его для ЧУЖИХ типов:
//! i32 (вес = само число по модулю), String и &str (вес = длина в байтах),
//! Vec<i32> (вес = сумма весов элементов). Orphan rule это разрешает: трейт - ваш.

pub trait Weight {
    /// Неотрицательный "вес" значения.
    fn weight(&self) -> u64;
}

impl Weight for i32 {
    fn weight(&self) -> u64 {
        todo!("модуль числа; у i32 есть .unsigned_abs() -> u32")
    }
}

impl Weight for String {
    fn weight(&self) -> u64 {
        todo!("длина в байтах")
    }
}

impl Weight for &str {
    fn weight(&self) -> u64 {
        todo!()
    }
}

impl Weight for Vec<i32> {
    fn weight(&self) -> u64 {
        todo!("сумма весов элементов")
    }
}
