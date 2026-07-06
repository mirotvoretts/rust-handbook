//! 01 (0x) — Инструменты Vec.
//!
//! Всё решается ОДНИМ вызовом готового метода Vec — без ручных циклов:
//! retain, extend, dedup, with_capacity.

/// Убирает нечётные на месте.
pub fn keep_even(v: &mut Vec<i32>) {
    todo!("retain")
}

/// Дозаписывает все элементы из среза.
pub fn append_all(v: &mut Vec<String>, extra: &[&str]) {
    todo!("extend с map или циклом... нет: extend(extra.iter().map(...)) — одним вызовом")
}

/// Схлопывает СОСЕДНИЕ дубликаты.
pub fn squash(v: &mut Vec<i32>) {
    todo!("dedup")
}

/// Вектор квадратов 1..=n БЕЗ реаллокаций (тест проверяет capacity).
pub fn squares(n: usize) -> Vec<u64> {
    todo!("with_capacity(n), затем push — или collect (он тоже угадывает размер)")
}
